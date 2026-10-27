//! `cargo xtask readiness`: the measured readiness number. Every dimension of
//! [`ROADMAP.md`](../../ROADMAP.md)'s weighted sum comes from a fixed pass/fail check set in
//! `docs/readiness/` (see its README), and the result is written to `docs/readiness.md`. A check
//! that wasn't run, or that is untested, doesn't pass.
//!
//! Documents are checked through `doc4me-cli`, one process per step under a timeout, so a crash
//! or a hang fails that document's stability check instead of stopping the run. Pages are compared
//! with the reference PDFs (made by Microsoft Word) with poppler's `pdfinfo` and `pdftotext`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use sha2::Digest;

/// The dimensions, their weights (as in `docs/target-app-parity.md`) and their check sets. A
/// dimension's score is the mean of its sets' pass rates.
pub const DIMENSIONS: &[(&str, f64, &[&str])] = &[
    ("Features", 0.30, &["ribbon", "dialogs"]),
    ("File formats", 0.20, &["corpus-open", "corpus-roundtrip"]),
    ("Layout and pagination", 0.15, &["corpus-pages", "corpus-breaks"]),
    ("UI/UX", 0.10, &["shortcuts"]),
    ("Stability", 0.10, &["corpus-stability", "stability"]),
    ("Performance", 0.05, &["performance"]),
    ("Localization", 0.04, &["languages"]),
    ("Platforms", 0.03, &["platforms"]),
    ("Ecosystem", 0.02, &["ecosystem"]),
    ("Hardware", 0.01, &["hardware"]),
];

/// The corpus size the number needs before it counts as final.
const CORPUS_TARGET: usize = 200;
/// A catalog covering this share of the interface's strings passes for its language.
const LANGUAGE_COVERAGE: f64 = 0.95;
/// Longest a `doc4me-cli` step may take on one document.
const STEP_TIMEOUT: Duration = Duration::from_secs(180);

/// One check set's outcome.
#[derive(Debug, Default)]
pub struct SetResult {
    pub name: String,
    pub passed: usize,
    pub total: usize,
    /// Checks that couldn't run (missing corpus file, untested manual row…), included in `total`.
    pub not_run: usize,
    /// Rows set aside until the owner asks for them (status `deferred`: checks only the owner's
    /// own hardware or Windows PC can make, builds the owner hasn't asked for, Microsoft
    /// services): included in `total` as not passing, left out of [`SetResult::rate_ahead`].
    pub deferred: usize,
    /// Failing or not-run checks: (id, reason).
    pub failing: Vec<(String, String)>,
}

impl SetResult {
    fn new(name: &str) -> Self {
        SetResult { name: name.into(), ..Default::default() }
    }
    fn pass(&mut self) {
        self.passed += 1;
        self.total += 1;
    }
    fn fail(&mut self, id: &str, why: impl Into<String>) {
        self.total += 1;
        self.failing.push((id.into(), why.into()));
    }
    fn skip(&mut self, id: &str, why: impl Into<String>) {
        self.not_run += 1;
        self.fail(id, why);
    }
    fn defer(&mut self, id: &str, why: impl Into<String>) {
        self.deferred += 1;
        self.fail(id, format!("deferred: {}", why.into()));
    }
    pub fn rate(&self) -> f64 {
        if self.total == 0 { 0.0 } else { self.passed as f64 / self.total as f64 }
    }
    /// The pass rate of the rows that aren't deferred (all of them deferred: nothing is left to do).
    pub fn rate_ahead(&self) -> f64 {
        let ahead = self.total - self.deferred;
        if self.total == 0 {
            0.0
        } else if ahead == 0 {
            1.0
        } else {
            self.passed as f64 / ahead as f64
        }
    }
}

struct Opts {
    corpus: bool,
    download: bool,
    cli: Option<PathBuf>,
    jobs: usize,
    write: bool,
}

pub fn run(root: &Path, args: &[&str]) -> Result<(), String> {
    let mut opts = Opts { corpus: false, download: false, cli: None, jobs: 0, write: true };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match *a {
            "--corpus" => opts.corpus = true,
            "--download" => opts.download = true,
            "--dry-run" => opts.write = false,
            "--cli" => opts.cli = Some(PathBuf::from(it.next().ok_or("--cli needs a path")?)),
            "--jobs" => opts.jobs = it.next().and_then(|n| n.parse().ok()).ok_or("--jobs needs a number")?,
            other => return Err(format!("readiness: unknown option {other}")),
        }
    }
    let dir = root.join("docs/readiness");
    let corpus_dir = root.join("corpus/readiness");
    let manifest = corpus_rows(read_tsv(&dir.join("corpus.tsv"))?)?;
    if opts.download {
        download(&manifest, &corpus_dir)?;
    }
    if opts.corpus && !["pdfinfo", "pdftotext"].iter().all(|t| Command::new(t).arg("-v").output().is_ok()) {
        return Err("--corpus compares pages with poppler's pdfinfo and pdftotext; install poppler-utils (Windows: poppler for Windows) first".into());
    }
    let cli = match opts.cli {
        Some(p) => p,
        None => build_cli(root)?,
    };
    let registry = Registry::load(&cli)?;
    let mut sets =
        vec![ribbon(&cli)?, dialogs(&read_tsv(&dir.join("dialogs.tsv"))?, &registry), shortcuts(&read_tsv(&dir.join("shortcuts.tsv"))?, &registry)];
    sets.push(languages(&read_tsv(&dir.join("languages.tsv"))?, &root.join("crates/ui-egui/src/i18n"))?);
    sets.extend(manual(&read_tsv(&dir.join("manual.tsv"))?));
    sets.push(performance(&read_tsv(&dir.join("performance.tsv"))?, &cli, &root.join("target/readiness/perf"))?);
    let jobs = if opts.jobs > 0 { opts.jobs } else { std::thread::available_parallelism().map_or(2, |n| n.get().min(8)) };
    let present = sets_from_corpus(&manifest, &corpus_dir, &cli, &root.join("target/readiness/corpus"), opts.corpus, jobs);
    let corpus_files = present.1;
    sets.extend(present.0);
    let report = render(&sets, corpus_files, manifest.len(), &commit(root), &today());
    if opts.write {
        let out = root.join("docs/readiness.md");
        std::fs::write(&out, &report).map_err(|e| format!("{}: {e}", out.display()))?;
        println!("wrote {}", out.display());
    }
    if let Some(line) = report.lines().find(|l| l.starts_with("**Measured readiness")) {
        println!("{line}");
    }
    Ok(())
}

// ---- check sets from files ----

/// A TSV file: a header row naming the columns, then one row per check; `#` lines are comments.
pub fn read_tsv(path: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(parse_tsv(&text))
}

pub fn parse_tsv(text: &str) -> Vec<BTreeMap<String, String>> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#'));
    let Some(header) = lines.next() else { return Vec::new() };
    let cols: Vec<&str> = header.split('\t').collect();
    lines.map(|l| cols.iter().zip(l.split('\t').chain(std::iter::repeat(""))).map(|(c, v)| (c.to_string(), v.trim().to_string())).collect()).collect()
}

fn col<'a>(row: &'a BTreeMap<String, String>, name: &str) -> &'a str {
    row.get(name).map_or("", String::as_str)
}

/// Rows checked by hand: `pass`, `fail`, `deferred` or `untested` (not passing).
fn manual_row(set: &mut SetResult, row: &BTreeMap<String, String>, what: &str) {
    let id = col(row, "id");
    match col(row, "status") {
        "pass" => set.pass(),
        "fail" => set.fail(id, format!("fail (checked by hand): {what}")),
        "deferred" => set.defer(id, what),
        _ => set.skip(id, format!("untested: {what}")),
    }
}

pub fn dialogs(rows: &[BTreeMap<String, String>], reg: &Registry) -> SetResult {
    let mut set = SetResult::new("dialogs");
    for row in rows {
        let (id, name, cmd) = (col(row, "id"), col(row, "word_dialog"), col(row, "command"));
        match col(row, "check") {
            "excluded" => {}
            "manual" => manual_row(&mut set, row, name),
            _ if cmd != "-" && reg.has(cmd) => set.pass(),
            _ if col(row, "status") == "deferred" => set.defer(id, format!("{name}: {}", col(row, "note"))),
            _ => set.fail(id, format!("{name}: no equivalent dialog")),
        }
    }
    set
}

pub fn shortcuts(rows: &[BTreeMap<String, String>], reg: &Registry) -> SetResult {
    let mut set = SetResult::new("shortcuts");
    for row in rows {
        let (id, keys, cmd) = (col(row, "id"), col(row, "word_keys"), col(row, "command"));
        let what = format!("{keys} — {}", col(row, "word_action"));
        match col(row, "check") {
            "excluded" | "duplicate" => {}
            "manual" => manual_row(&mut set, row, &what),
            _ if col(row, "chord").split('|').any(|c| reg.bound(c.trim(), cmd)) => set.pass(),
            _ if col(row, "status") == "deferred" => set.defer(id, what),
            _ => set.fail(id, what),
        }
    }
    set
}

pub fn manual(rows: &[BTreeMap<String, String>]) -> Vec<SetResult> {
    let mut by: BTreeMap<String, SetResult> = BTreeMap::new();
    for name in ["stability", "platforms", "ecosystem", "hardware"] {
        by.insert(name.into(), SetResult::new(name));
    }
    for row in rows {
        if let Some(set) = by.get_mut(col(row, "set")) {
            manual_row(set, row, col(row, "check"));
        }
    }
    by.into_values().collect()
}

/// A language passes when Doc4Me has a catalog for it covering [`LANGUAGE_COVERAGE`] of the
/// strings any catalog translates (English is the source language and always passes).
pub fn languages(rows: &[BTreeMap<String, String>], i18n: &Path) -> Result<SetResult, String> {
    let mut catalogs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in std::fs::read_dir(i18n).map_err(|e| format!("{}: {e}", i18n.display()))?.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|x| x == "tsv")
            && let Some(code) = path.file_stem().and_then(|s| s.to_str())
        {
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            catalogs.insert(code.into(), catalog_keys(&text));
        }
    }
    let all: BTreeSet<&String> = catalogs.values().flatten().collect();
    let mut set = SetResult::new("languages");
    for row in rows {
        let (id, lang, code) = (col(row, "id"), col(row, "language"), col(row, "catalog"));
        if code == "en" {
            set.pass();
            continue;
        }
        match catalogs.get(code) {
            Some(keys) => {
                let share = if all.is_empty() { 0.0 } else { keys.len() as f64 / all.len() as f64 };
                if share >= LANGUAGE_COVERAGE {
                    set.pass();
                } else {
                    set.fail(id, format!("{lang}: catalog `{code}` covers {:.1}%", share * 100.0));
                }
            }
            None => set.fail(id, format!("{lang}: no interface catalog")),
        }
    }
    Ok(set)
}

/// The keys of an interface catalog (`\tkey\tvalue` lines).
pub fn catalog_keys(text: &str) -> BTreeSet<String> {
    text.lines().filter_map(|l| l.strip_prefix('\t')).filter_map(|l| l.split_once('\t')).map(|(k, _)| k.to_string()).collect()
}

// ---- the command registry ----

/// Commands and their shortcuts, from `doc4me-cli commands --json`.
pub struct Registry {
    shortcuts: BTreeMap<String, Vec<String>>,
}

impl Registry {
    fn load(cli: &Path) -> Result<Self, String> {
        let out = Command::new(cli).args(["commands", "--json"]).output().map_err(|e| format!("{}: {e}", cli.display()))?;
        if !out.status.success() {
            return Err(format!("doc4me-cli commands failed: {}", String::from_utf8_lossy(&out.stderr)));
        }
        let list: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|e| format!("commands --json: {e}"))?;
        Ok(Self::from_json(&list))
    }

    pub fn from_json(list: &serde_json::Value) -> Self {
        let mut shortcuts = BTreeMap::new();
        for c in list.as_array().into_iter().flatten() {
            let id = c.get("id").and_then(|v| v.as_str()).unwrap_or_default();
            let keys = c.get("shortcut").and_then(|v| v.as_str()).unwrap_or_default();
            shortcuts.insert(id.to_string(), keys.split('/').filter_map(normalize).collect());
        }
        Registry { shortcuts }
    }

    pub fn has(&self, id: &str) -> bool {
        self.shortcuts.contains_key(id)
    }

    /// Whether `chord` runs `command`: bound to it, or (with Shift) a caret move it extends, as
    /// the app's key handling does (`crates/ui-egui/src/keys.rs`).
    pub fn bound(&self, chord: &str, command: &str) -> bool {
        let (Some(want), Some(keys)) = (normalize(chord), self.shortcuts.get(command)) else { return false };
        if keys.contains(&want) {
            return true;
        }
        command.starts_with("caret.") && want.contains("Shift+") && normalize(&want.replacen("Shift+", "", 1)).is_some_and(|k| keys.contains(&k))
    }
}

/// A key chord in one spelling: modifiers in the order `Mod+Ctrl+Alt+Shift+`, then the key (a
/// single character upper-cased). `None` for an empty chord.
pub fn normalize(chord: &str) -> Option<String> {
    let parts: Vec<&str> = chord.trim().split('+').map(str::trim).collect();
    let (key, mods) = parts.split_last()?;
    let key = match key.chars().count() {
        0 if chord.trim().ends_with('+') => "+".to_string(),
        0 => return None,
        1 => key.to_uppercase(),
        _ => key.to_string(),
    };
    let mut out: Vec<&str> = ["Mod", "Ctrl", "Alt", "Shift"].into_iter().filter(|m| mods.contains(m)).collect();
    if mods.iter().any(|m| !["Mod", "Ctrl", "Alt", "Shift", ""].contains(m)) {
        return None;
    }
    let key = key.as_str();
    out.push(key);
    Some(out.join("+"))
}

// ---- doc4me-cli ----

fn build_cli(root: &Path) -> Result<PathBuf, String> {
    let status = crate::cargo().args(["build", "-q", "--release", "-p", "doc4me-cli"]).status().map_err(|e| format!("cargo build: {e}"))?;
    if !status.success() {
        return Err("cargo build --release -p doc4me-cli failed".into());
    }
    Ok(root.join("target/release").join(if cfg!(windows) { "doc4me-cli.exe" } else { "doc4me-cli" }))
}

fn ribbon(cli: &Path) -> Result<SetResult, String> {
    let out = Command::new(cli).arg("parity").output().map_err(|e| format!("{}: {e}", cli.display()))?;
    let p: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|e| format!("parity: {e}"))?;
    let mut set = SetResult::new("ribbon");
    for tab in p.get("tabs").and_then(|t| t.as_array()).into_iter().flatten() {
        let live = tab.get("live").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        set.passed += live;
        set.total += live;
        for m in tab.get("missing").and_then(|v| v.as_array()).into_iter().flatten() {
            set.fail(tab.get("tab").and_then(|v| v.as_str()).unwrap_or(""), m.as_str().unwrap_or(""));
        }
    }
    Ok(set)
}

/// How a step ended.
#[derive(Debug)]
pub enum Outcome {
    Ok(String),
    /// Exited with an error (a message, not a crash).
    Failed(String),
    /// Panicked or was killed by a signal.
    Crashed(String),
    TimedOut,
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outcome::Ok(_) => f.write_str("ok"),
            Outcome::Failed(e) => write!(f, "error: {e}"),
            Outcome::Crashed(e) => write!(f, "crash: {e}"),
            Outcome::TimedOut => write!(f, "no result after {} s", STEP_TIMEOUT.as_secs()),
        }
    }
}

impl Outcome {
    fn stable(&self) -> bool {
        matches!(self, Outcome::Ok(_) | Outcome::Failed(_))
    }
}

/// Run `cmd` with its output in files under `work`, killing it after `timeout`.
fn run_step(mut cmd: Command, work: &Path, name: &str, timeout: Duration) -> Outcome {
    let (out_path, err_path) = (work.join(format!("{name}.out")), work.join(format!("{name}.err")));
    let (Ok(out), Ok(err)) = (File::create(&out_path), File::create(&err_path)) else { return Outcome::Failed("can't write step output".into()) };
    let mut child = match cmd.stdin(Stdio::null()).stdout(out).stderr(err).spawn() {
        Ok(c) => c,
        Err(e) => return Outcome::Failed(format!("spawn: {e}")),
    };
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if start.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Outcome::TimedOut;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(e) => return Outcome::Failed(format!("wait: {e}")),
        }
    };
    let stderr = std::fs::read_to_string(&err_path).unwrap_or_default();
    let stdout = std::fs::read_to_string(&out_path).unwrap_or_default();
    match status.code() {
        Some(0) => Outcome::Ok(stdout),
        Some(101) => Outcome::Crashed(first_line(&stderr)),
        Some(_) if stderr.contains("panicked") => Outcome::Crashed(first_line(&stderr)),
        Some(_) => Outcome::Failed(first_line(&stderr)),
        None => Outcome::Crashed(format!("killed by a signal: {}", first_line(&stderr))),
    }
}

fn first_line(s: &str) -> String {
    let line = s.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim();
    line.chars().take(160).collect()
}

fn cli_cmd(cli: &Path, args: &[&str]) -> Command {
    let mut c = Command::new(cli);
    c.args(args);
    c
}

// ---- performance ----

/// Generated documents of a given number of paragraphs, timed by `doc4me-cli info`'s own
/// `layoutMs` (open time excluded) and by the whole command (open, lay out, export); rows measured
/// `manual` (typing and scrolling in the window) carry a hand-checked `status`.
fn performance(rows: &[BTreeMap<String, String>], cli: &Path, work: &Path) -> Result<SetResult, String> {
    std::fs::create_dir_all(work).map_err(|e| format!("{}: {e}", work.display()))?;
    let mut set = SetResult::new("performance");
    for row in rows {
        let id = col(row, "id");
        if col(row, "measure") == "manual" {
            manual_row(&mut set, row, col(row, "check"));
            continue;
        }
        let paras: usize = col(row, "paragraphs").parse().map_err(|_| format!("performance {id}: paragraphs"))?;
        let limit: f64 = col(row, "threshold_ms").parse().map_err(|_| format!("performance {id}: threshold_ms"))?;
        let doc = work.join(format!("perf-{paras}.docx"));
        if !doc.exists() {
            let txt = work.join(format!("perf-{paras}.txt"));
            std::fs::write(&txt, sample_text(paras)).map_err(|e| format!("{}: {e}", txt.display()))?;
            if let o @ (Outcome::Failed(_) | Outcome::Crashed(_) | Outcome::TimedOut) =
                run_step(cli_cmd(cli, &["convert", &txt.to_string_lossy(), &doc.to_string_lossy()]), work, "gen", STEP_TIMEOUT)
            {
                set.fail(id, format!("couldn't make the test document: {o}"));
                continue;
            }
        }
        let mut times = Vec::new();
        for _ in 0..3 {
            let t = Instant::now();
            let o = match col(row, "measure") {
                "pdf" => {
                    run_step(cli_cmd(cli, &["convert", &doc.to_string_lossy(), &work.join("perf.pdf").to_string_lossy()]), work, "perf", STEP_TIMEOUT)
                }
                _ => run_step(cli_cmd(cli, &["info", &doc.to_string_lossy()]), work, "perf", STEP_TIMEOUT),
            };
            let ms = match (&o, col(row, "measure")) {
                (Outcome::Ok(out), "layout") => {
                    serde_json::from_str::<serde_json::Value>(out).ok().and_then(|v| v.get("layoutMs").and_then(|x| x.as_f64()))
                }
                (Outcome::Ok(_), _) => Some(t.elapsed().as_secs_f64() * 1000.0),
                _ => None,
            };
            match ms {
                Some(ms) => times.push(ms),
                None => break,
            }
        }
        times.sort_by(f64::total_cmp);
        match times.get(1) {
            Some(&median) if median <= limit => set.pass(),
            Some(&median) => set.fail(id, format!("{}: {median:.0} ms (limit {limit:.0} ms)", col(row, "check"))),
            None => set.fail(id, format!("{}: didn't finish", col(row, "check"))),
        }
    }
    Ok(set)
}

/// Plain English paragraphs (about 90 words each) for the timing documents.
pub fn sample_text(paragraphs: usize) -> String {
    const P: &str = "Every Thursday evening the old print shop on Alder Street fills with people who make things. Painters set up by the north windows, letterpress regulars claim the back room, and somebody always brings bread. This handbook collects what we have learned about running a shared studio: the routines that keep it pleasant, the tools we pool, and the small agreements that let thirty strangers become a crew. Rent is the obvious reason, but it is not the best one.";
    let mut s = String::with_capacity(paragraphs * (P.len() + 2));
    for i in 0..paragraphs {
        if i % 20 == 0 {
            s.push_str(&format!("Section {}\n\n", i / 20 + 1));
        }
        s.push_str(P);
        s.push_str("\n\n");
    }
    s
}

// ---- the corpus ----

fn sha256_file(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(sha2::Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect())
}

/// Fetch the manifest's files into `dir`, keeping only those whose SHA-256 matches.
fn download(manifest: &[BTreeMap<String, String>], dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let (mut got, mut bad) = (0, 0);
    for row in manifest {
        let id = col(row, "id");
        for (ext, url, sha) in [("docx", col(row, "docx_url"), col(row, "docx_sha256")), ("pdf", col(row, "pdf_url"), col(row, "pdf_sha256"))] {
            let path = dir.join(format!("{id}.{ext}"));
            if sha256_file(&path).as_deref() == Some(sha) {
                continue;
            }
            if !url.starts_with("https://") {
                bad += 1;
                continue;
            }
            let ok = Command::new("curl")
                .args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-filesize", "52428800", "-o"])
                .arg(&path)
                .arg(url)
                .status()
                .is_ok_and(|s| s.success());
            if ok && sha256_file(&path).as_deref() == Some(sha) {
                got += 1;
            } else {
                let _ = std::fs::remove_file(&path);
                bad += 1;
                eprintln!("{id}.{ext}: download failed or SHA-256 differs ({url})");
            }
        }
    }
    println!("corpus: {got} files downloaded, {bad} failed, into {}", dir.display());
    Ok(())
}

/// One corpus document's results.
#[derive(Debug)]
struct DocResult {
    id: String,
    open: Result<(), String>,
    roundtrip: Result<(), String>,
    pages: Result<(), String>,
    breaks: Result<(), String>,
    stable: Result<(), String>,
}

/// The manifest rows the corpus sets count: a row whose reference PDF turned out not to be the
/// same document stays in `corpus.tsv` with `check` = `excluded` and isn't fetched or counted.
/// An id names the files and the work folder that is emptied before each check, so it must be a
/// plain name.
fn corpus_rows(manifest: Vec<BTreeMap<String, String>>) -> Result<Vec<BTreeMap<String, String>>, String> {
    if let Some(bad) =
        manifest.iter().map(|r| col(r, "id")).find(|id| id.is_empty() || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'))
    {
        return Err(format!("corpus.tsv: id {bad:?} must be letters, digits, '-' or '_'"));
    }
    Ok(manifest.into_iter().filter(|r| col(r, "check") != "excluded").collect())
}

/// The five corpus sets, and how many documents were available. Without `run`, every document
/// counts as not run.
fn sets_from_corpus(manifest: &[BTreeMap<String, String>], dir: &Path, cli: &Path, work: &Path, run: bool, jobs: usize) -> (Vec<SetResult>, usize) {
    let names = ["corpus-open", "corpus-roundtrip", "corpus-pages", "corpus-breaks", "corpus-stability"];
    let mut sets: Vec<SetResult> = names.iter().map(|n| SetResult::new(n)).collect();
    let mut todo = Vec::new();
    for row in manifest {
        let id = col(row, "id");
        let (docx, pdf) = (dir.join(format!("{id}.docx")), dir.join(format!("{id}.pdf")));
        let present = sha256_file(&docx).as_deref() == Some(col(row, "docx_sha256")) && sha256_file(&pdf).as_deref() == Some(col(row, "pdf_sha256"));
        if run && present {
            todo.push((id.to_string(), docx, pdf));
        } else {
            let why = if present { "not run (cargo xtask readiness --corpus)" } else { "file missing (cargo xtask readiness --download)" };
            for s in &mut sets {
                s.skip(id, why);
            }
        }
    }
    let available = todo.len();
    let queue = Mutex::new(todo);
    let results = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let next = queue.lock().ok().and_then(|mut q| q.pop());
                    let Some((id, docx, pdf)) = next else { break };
                    let r = check_document(&id, &docx, &pdf, cli, &work.join(&id));
                    if let Ok(mut all) = results.lock() {
                        all.push(r);
                    }
                }
            });
        }
    });
    let mut results = results.into_inner().unwrap_or_default();
    results.sort_by(|a, b| a.id.cmp(&b.id));
    for r in &results {
        for (set, res) in sets.iter_mut().zip([&r.open, &r.roundtrip, &r.pages, &r.breaks, &r.stable]) {
            match res {
                Ok(()) => set.pass(),
                Err(e) => set.fail(&r.id, e.clone()),
            }
        }
    }
    (sets, available)
}

fn check_document(id: &str, docx: &Path, reference: &Path, cli: &Path, work: &Path) -> DocResult {
    let unchecked = || Err::<(), String>("not checked".into());
    let mut r = DocResult { id: id.into(), open: unchecked(), roundtrip: unchecked(), pages: unchecked(), breaks: unchecked(), stable: unchecked() };
    let _ = std::fs::remove_dir_all(work);
    if let Err(e) = std::fs::create_dir_all(work) {
        let e = format!("work folder: {e}");
        (r.open, r.roundtrip, r.pages, r.breaks, r.stable) = (Err(e.clone()), Err(e.clone()), Err(e.clone()), Err(e.clone()), Err(e));
        return r;
    }
    let path = |p: &Path| p.to_string_lossy().to_string();
    let mut unstable = Vec::new();
    let mut step = |args: &[String], name: &str| {
        let o = run_step(cli_cmd(cli, &args.iter().map(String::as_str).collect::<Vec<_>>()), work, name, STEP_TIMEOUT);
        if !o.stable() {
            unstable.push(format!("{name}: {o}"));
        }
        o
    };
    // Open (and lay out).
    let info = step(&["info".into(), path(docx)], "info");
    r.open = match &info {
        Outcome::Ok(_) => Ok(()),
        o => Err(format!("open: {o}")),
    };
    // Save and reopen: the text and the structure stay.
    let rt = work.join("roundtrip.docx");
    r.roundtrip = match step(&["convert".into(), path(docx), path(&rt)], "save") {
        Outcome::Ok(_) => {
            let (a, b) = (step(&["text".into(), path(docx)], "text-a"), step(&["text".into(), path(&rt)], "text-b"));
            let (ia, ib) = (step(&["inspect".into(), path(docx)], "inspect-a"), step(&["inspect".into(), path(&rt)], "inspect-b"));
            match (a, b, ia, ib) {
                (Outcome::Ok(a), Outcome::Ok(b), Outcome::Ok(ia), Outcome::Ok(ib)) => {
                    let print = |s: &str| serde_json::from_str::<serde_json::Value>(s).map(|v| fingerprint(&v)).unwrap_or_default();
                    if words(&a) != words(&b) {
                        Err("text changed after save and reopen".into())
                    } else {
                        structure_change(&print(&ia), &print(&ib)).map_or(Ok(()), Err)
                    }
                }
                _ => Err("reading the saved copy failed".into()),
            }
        }
        o => Err(format!("save: {o}")),
    };
    // Pages against the reference PDF.
    let ours = work.join("doc4me.pdf");
    let (pages, breaks) = match step(&["convert".into(), path(docx), path(&ours)], "pdf") {
        Outcome::Ok(_) => compare_pages(&ours, reference),
        o => (Err(format!("export: {o}")), Err("no PDF".into())),
    };
    (r.pages, r.breaks) = (pages, breaks);
    r.stable = if unstable.is_empty() { Ok(()) } else { Err(unstable.join("; ")) };
    r
}

/// What a document is made of, from `doc4me-cli inspect`: how many blocks, objects and runs of each
/// `type` and `style` there are anywhere in its body (paragraphs, tables, pictures, fields,
/// bookmarks, shapes, paragraph and character styles…), its parts by `kind` (headers, footers,
/// text boxes, notes, comments) and its sections.
pub fn fingerprint(doc: &serde_json::Value) -> BTreeMap<String, usize> {
    fn walk(v: &serde_json::Value, out: &mut BTreeMap<String, usize>) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m {
                    if let (Some(name), "type" | "style") = (x.as_str(), k.as_str()) {
                        *out.entry(format!("{k} {name}")).or_default() += 1;
                    }
                    walk(x, out);
                }
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            _ => {}
        }
    }
    let mut out = BTreeMap::new();
    if let Some(blocks) = doc.get("blocks") {
        walk(blocks, &mut out);
    }
    for part in doc.get("parts").and_then(|p| p.as_array()).into_iter().flatten() {
        *out.entry(format!("part {}", part.get("kind").and_then(|k| k.as_str()).unwrap_or("?"))).or_default() += 1;
    }
    out.insert("sections".into(), doc.get("sections").and_then(|s| s.as_array()).map_or(0, Vec::len));
    out
}

/// The first difference between two [`fingerprint`]s, as "type image: 2 → 1 …".
pub fn structure_change(before: &BTreeMap<String, usize>, after: &BTreeMap<String, usize>) -> Option<String> {
    before.keys().chain(after.keys()).find_map(|k| {
        let (a, b) = (before.get(k).copied().unwrap_or(0), after.get(k).copied().unwrap_or(0));
        (a != b).then(|| format!("{k}: {a} → {b} after save and reopen"))
    })
}

/// Page count, then the first and last word of every page.
fn compare_pages(ours: &Path, reference: &Path) -> (Result<(), String>, Result<(), String>) {
    let (Some(a), Some(b)) = (pdf_pages(ours), pdf_pages(reference)) else {
        return (Err("pdfinfo couldn't read a PDF (is poppler installed?)".into()), Err("no page count".into()));
    };
    if a != b {
        return (Err(format!("{a} pages, Word {b}")), Err("page count differs".into()));
    }
    for page in 1..=a {
        let (x, y) = (page_words(ours, page), page_words(reference, page));
        let ends = |w: &[String]| (w.first().cloned(), w.last().cloned());
        if ends(&x) != ends(&y) {
            return (Ok(()), Err(format!("page {page} starts or ends differently")));
        }
    }
    (Ok(()), Ok(()))
}

fn pdf_pages(pdf: &Path) -> Option<usize> {
    let out = Command::new("pdfinfo").arg(pdf).output().ok()?;
    String::from_utf8_lossy(&out.stdout).lines().find_map(|l| l.strip_prefix("Pages:")).and_then(|n| n.trim().parse().ok())
}

fn page_words(pdf: &Path, page: usize) -> Vec<String> {
    let p = page.to_string();
    let out = Command::new("pdftotext").args(["-q", "-f", &p, "-l", &p]).arg(pdf).arg("-").output();
    out.map(|o| words(&String::from_utf8_lossy(&o.stdout))).unwrap_or_default()
}

/// Words for comparing text: letters and digits, lower-cased (punctuation, spacing, ligatures and
/// hyphenation points aside).
pub fn words(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect()
}

// ---- the report ----

pub fn score(sets: &[SetResult]) -> (f64, Vec<(&'static str, f64, f64)>) {
    score_with(sets, SetResult::rate)
}

/// [`score`] without the deferred rows: the share of the work not waiting on the owner.
pub fn score_ahead(sets: &[SetResult]) -> (f64, Vec<(&'static str, f64, f64)>) {
    score_with(sets, SetResult::rate_ahead)
}

fn score_with(sets: &[SetResult], rate: fn(&SetResult) -> f64) -> (f64, Vec<(&'static str, f64, f64)>) {
    let mut total = 0.0;
    let mut dims = Vec::new();
    for (name, weight, names) in DIMENSIONS {
        let rates: Vec<f64> = names.iter().map(|n| sets.iter().find(|s| s.name == *n).map_or(0.0, rate)).collect();
        let dim = rates.iter().sum::<f64>() / rates.len().max(1) as f64;
        total += weight * dim;
        dims.push((*name, *weight, dim));
    }
    (total, dims)
}

fn render(sets: &[SetResult], corpus_run: usize, corpus_listed: usize, commit: &str, date: &str) -> String {
    let (total, dims) = score(sets);
    let (ahead, dims_ahead) = score_ahead(sets);
    let deferred: usize = sets.iter().map(|s| s.deferred).sum();
    let mut md = String::new();
    md.push_str("# Measured readiness\n\n");
    md.push_str(&format!(
        "> Generated by `cargo xtask readiness` on {date} at commit `{commit}`; don't edit by hand. Check sets and rules: [`readiness/README.md`](readiness/README.md).\n\n"
    ));
    md.push_str(&format!("**Measured readiness: {:.1}%**\n\n", total * 100.0));
    md.push_str(&format!(
        "**Without the deferred rows: {:.1}%** — {deferred} rows wait until the owner asks for them (checks only the owner's own hardware or Windows PC can make, builds the owner hasn't asked for, Microsoft services); they count as not passing in the number above.\n\n",
        ahead * 100.0
    ));
    if corpus_run < CORPUS_TARGET {
        md.push_str(&format!(
            "Not final: the real-world corpus checks ran on {corpus_run} documents ({corpus_listed} listed in `corpus.tsv`); the number counts as measured from {CORPUS_TARGET}.\n\n"
        ));
    }
    md.push_str("| Dimension | Weight | Score | Without deferred | Check sets (passed / checked) |\n|---|---|---|---|---|\n");
    for ((name, weight, dim), (_, _, dim_ahead)) in dims.iter().zip(&dims_ahead) {
        let names = DIMENSIONS.iter().find(|d| d.0 == *name).map_or(&[][..], |d| d.2);
        let detail: Vec<String> = names
            .iter()
            .map(|n| match sets.iter().find(|s| s.name == *n) {
                Some(s) if s.not_run > 0 || s.deferred > 0 => {
                    let mut why = Vec::new();
                    if s.not_run > 0 {
                        why.push(format!("{} not run", s.not_run));
                    }
                    if s.deferred > 0 {
                        why.push(format!("{} deferred", s.deferred));
                    }
                    format!("{n} {}/{} ({})", s.passed, s.total, why.join(", "))
                }
                Some(s) => format!("{n} {}/{}", s.passed, s.total),
                None => format!("{n} —"),
            })
            .collect();
        md.push_str(&format!("| {name} | {weight:.2} | {:.1}% | {:.1}% | {} |\n", dim * 100.0, dim_ahead * 100.0, detail.join("; ")));
    }
    md.push_str("\n## Not passing\n\nThe first 40 of each set; the rest are counted above.\n");
    for s in sets.iter().filter(|s| !s.failing.is_empty()) {
        md.push_str(&format!("\n### {} ({} of {})\n\n", s.name, s.failing.len(), s.total));
        for (id, why) in s.failing.iter().take(40) {
            md.push_str(&format!("- `{id}` {}\n", why.replace('\n', " ").replace('|', "/")));
        }
    }
    md
}

fn commit(root: &Path) -> String {
    let out = Command::new("git").current_dir(root).args(["rev-parse", "--short", "HEAD"]).output();
    let hash = out.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    let dirty =
        Command::new("git").current_dir(root).args(["status", "--porcelain", "--untracked-files=no"]).output().is_ok_and(|o| !o.stdout.is_empty());
    if dirty { format!("{hash}+changes") } else { hash }
}

/// Today's date (UTC), as YYYY-MM-DD.
fn today() -> String {
    let days = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() / 86_400) as i64;
    civil_date(days)
}

/// The proleptic Gregorian date of a day count since 1970-01-01 (Howard Hinnant's algorithm).
pub fn civil_date(days: i64) -> String {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> Registry {
        Registry::from_json(&serde_json::json!([
            {"id": "format.bold", "shortcut": "Mod+B"},
            {"id": "format.growFont", "shortcut": "Mod+Shift+. / Mod+Shift+>"},
            {"id": "caret.wordLeft", "shortcut": "Mod+Left / Alt+Left"},
            {"id": "edit.goto", "shortcut": "Mod+Alt+G / F5"},
            {"id": "insert.table", "shortcut": ""},
        ]))
    }

    #[test]
    fn chords_compare_in_one_spelling() {
        assert_eq!(normalize("Shift+Mod+b").as_deref(), Some("Mod+Shift+B"));
        assert_eq!(normalize(" Alt + F5 ").as_deref(), Some("Alt+F5"));
        assert_eq!(normalize("Mod+Shift++").as_deref(), Some("Mod+Shift++"));
        assert_eq!(normalize("Hyper+B"), None);
        assert_eq!(normalize(""), None);
        let r = registry();
        assert!(r.bound("Mod+B", "format.bold"));
        assert!(r.bound("Mod+Shift+>", "format.growFont"));
        assert!(r.bound("F5", "edit.goto"));
        assert!(!r.bound("Mod+G", "edit.goto"));
        // Shift extends a caret move bound to the chord without it; not other commands.
        assert!(r.bound("Mod+Shift+Left", "caret.wordLeft"));
        assert!(!r.bound("Mod+Shift+B", "format.bold"));
        assert!(!r.bound("Mod+B", "missing.command"));
        assert!(r.has("insert.table") && !r.has("insert.nothing"));
    }

    #[test]
    fn check_sets_score_what_passes_and_count_untested_as_not_passing() {
        let r = registry();
        let rows = parse_tsv(
            "# comment\nid\tword_keys\tword_action\tchord\tcommand\tcheck\tstatus\n\
             ks-1\tCtrl+B\tBold\tMod+B\tformat.bold\tregistry\t\n\
             ks-2\tCtrl+G\tGo To\tMod+G\tedit.goto\tregistry\t\n\
             ks-3\tCtrl+B\tBold\t\t\tduplicate\tks-1\n\
             ks-4\tAlt,H\tHome tab\t\t-\tmanual\tpass\n\
             ks-5\tAlt,Q\tSearch\t\t-\tmanual\tuntested\n\
             ks-6\tAlt+F11\tVBA\t\t\texcluded\t\n",
        );
        let s = shortcuts(&rows, &r);
        assert_eq!((s.passed, s.total, s.not_run), (2, 4, 1));
        assert_eq!(s.failing.iter().map(|f| f.0.as_str()).collect::<Vec<_>>(), ["ks-2", "ks-5"]);
        let d = dialogs(
            &parse_tsv("id\tword_dialog\tcommand\tcheck\tstatus\nd1\tTable\tinsert.table\tregistry\t\nd2\tX\t-\tregistry\t\nd3\tMac\t\texcluded\t\n"),
            &r,
        );
        assert_eq!((d.passed, d.total), (1, 2));
        let m = manual(&parse_tsv("id\tset\tcheck\tstatus\nm1\tplatforms\tWindows\tpass\nm2\thardware\tPen\tfail\nm3\tnowhere\tX\tpass\n"));
        let get = |n: &str| m.iter().find(|s| s.name == n).map(|s| (s.passed, s.total)).unwrap();
        assert_eq!((get("platforms"), get("hardware"), get("stability")), ((1, 1), (0, 1), (0, 0)));
    }

    #[test]
    fn the_number_is_the_weighted_mean_of_the_dimensions() {
        assert!((DIMENSIONS.iter().map(|d| d.1).sum::<f64>() - 1.0).abs() < 1e-9);
        let mut ribbon = SetResult::new("ribbon");
        (0..4).for_each(|_| ribbon.pass());
        let mut dialogs = SetResult::new("dialogs");
        dialogs.pass();
        dialogs.fail("d", "no");
        // Features = mean(100%, 50%) = 75%, weight 0.30; every other set is absent (0).
        let (total, dims) = score(&[ribbon, dialogs]);
        assert!((total - 0.225).abs() < 1e-9, "{total}");
        assert!((dims[0].2 - 0.75).abs() < 1e-9);
        let md = render(&[], 3, 5, "abc", "2026-10-25");
        assert!(md.contains("**Measured readiness: 0.0%**") && md.contains("ran on 3 documents (5 listed"), "{md}");
    }

    /// Deferred rows count as not passing in the measured number and are left out of the second.
    #[test]
    fn deferred_rows_are_left_out_of_the_second_number() {
        let rows = parse_tsv(
            "id\tset\tcheck\tstatus\nh1\thardware\tprinter\tpass\nh2\thardware\tpen\tdeferred\nh3\thardware\tscanner\tfail\nh4\thardware\tmic\tdeferred\n",
        );
        let sets = manual(&rows);
        let hw = sets.iter().find(|s| s.name == "hardware").unwrap();
        assert_eq!((hw.passed, hw.total, hw.deferred), (1, 4, 2));
        assert!((hw.rate() - 0.25).abs() < 1e-9 && (hw.rate_ahead() - 0.5).abs() < 1e-9);
        let d = dialogs(
            &parse_tsv(
                "id\tword_dialog\tcommand\tcheck\tstatus\tnote\nd1\tTable\tinsert.table\tregistry\t\t\nd2\tMail\t-\tregistry\tdeferred\tOutlook\n",
            ),
            &registry(),
        );
        assert_eq!((d.passed, d.total, d.deferred), (1, 2, 1));
        assert!(d.failing.iter().any(|(id, why)| id == "d2" && why.starts_with("deferred: Mail: Outlook")));
        let md = render(&sets, 3, 5, "abc", "2026-10-27");
        assert!(md.contains("**Without the deferred rows: ") && md.contains("hardware 1/4 (2 deferred)"), "{md}");
        // Every row deferred: nothing left to do counts as done.
        let mut all = SetResult::new("platforms");
        all.defer("p", "arm64");
        assert_eq!((all.rate(), all.rate_ahead()), (0.0, 1.0));
    }

    #[test]
    fn excluded_corpus_rows_are_not_counted() {
        let rows = corpus_rows(parse_tsv(
            "id\tdocx_url\tcheck\tnote\na\thttps://x/a.docx\t\t\nb\thttps://x/b.docx\texcluded\tother edition\nc\thttps://x/c.docx\n",
        ))
        .unwrap();
        assert_eq!(rows.iter().map(|r| col(r, "id")).collect::<Vec<_>>(), ["a", "c"]);
        for bad in ["..", "", "a/b", "a b"] {
            assert!(corpus_rows(parse_tsv(&format!("id\tdocx_url\n{bad}\thttps://x/a.docx\n"))).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn structure_is_compared_by_kind_and_count() {
        let doc = |pics: usize| {
            let objects: Vec<serde_json::Value> = (0..pics).map(|_| serde_json::json!({"type": "image"})).collect();
            serde_json::json!({
                "blocks": [{"type": "paragraph", "props": {"style": "Heading1"}, "objects": objects, "runs": [{"props": {"style": "Strong"}}]},
                           {"type": "table", "rows": [{"cells": [{"blocks": [{"type": "paragraph"}]}]}]}],
                "parts": [{"kind": "header"}, {"kind": "footer"}, {"kind": "footer"}],
                "sections": [{}, {}],
            })
        };
        let f = fingerprint(&doc(2));
        assert_eq!(f.get("type paragraph"), Some(&2));
        assert_eq!(f.get("type table"), Some(&1));
        assert_eq!(f.get("type image"), Some(&2));
        assert_eq!(f.get("style Heading1"), Some(&1));
        assert_eq!(f.get("style Strong"), Some(&1));
        assert_eq!(f.get("part footer"), Some(&2));
        assert_eq!(f.get("sections"), Some(&2));
        assert_eq!(structure_change(&f, &fingerprint(&doc(2))), None);
        assert_eq!(structure_change(&f, &fingerprint(&doc(1))).as_deref(), Some("type image: 2 → 1 after save and reopen"));
    }

    #[test]
    fn hand_checked_performance_rows_need_no_run() {
        let work = std::env::temp_dir().join(format!("xtask-readiness-{}", std::process::id()));
        let rows =
            parse_tsv("id\tcheck\tparagraphs\tmeasure\tthreshold_ms\tstatus\np1\tTyping\t\tmanual\t\tpass\np2\tScrolling\t\tmanual\t\tuntested\n");
        let s = performance(&rows, Path::new("/nonexistent/doc4me-cli"), &work).unwrap();
        assert_eq!((s.passed, s.total, s.not_run), (1, 2, 1));
        let _ = std::fs::remove_dir_all(&work);
    }

    #[test]
    fn catalogs_dates_and_words() {
        let keys = catalog_keys("# Thai\n\tOpen\tเปิด\n\tSave\tบันทึก\nnot a row\n");
        assert_eq!(keys.into_iter().collect::<Vec<_>>(), ["Open", "Save"]);
        assert_eq!(civil_date(0), "1970-01-01");
        assert_eq!(civil_date(20_386), "2025-10-25");
        assert_eq!(civil_date(-1), "1969-12-31");
        assert_eq!(words("Hello, wor-ld!\n ﬁne 12"), ["hello", "wor", "ld", "ﬁne", "12"]);
        let t = sample_text(25);
        assert!(t.starts_with("Section 1\n\nEvery") && t.contains("Section 2\n\n"));
    }
}
