//! `cargo xtask window <check>`: the readiness hand checks that need the app's window
//! (`docs/readiness/manual.tsv`, `performance.tsv`), run against a real Doc4Me window through its
//! control channel (`docs/control-protocol.md`). Each check starts the app with a settings folder
//! of its own, drives it as a user would (keys, text, wheel, touch), prints what it measured and
//! writes it to `<out>/<check>.json`; it fails when the row's condition doesn't hold.
//!
//! On Linux without a display, run it under `xvfb-run -a`. The checks:
//!
//! - `soak` (stab-001): `--hours H` (default 8) of editing with AutoSave on; every
//!   `--checkpoint M` minutes (default 15) the file on disk must hold what the window shows; then
//!   the app is killed (SIGKILL / TerminateProcess) and the file must open with everything typed.
//!   The run's progress is kept in `<out>/soak/state.json` at each checkpoint; `--resume` goes on
//!   from it after the machine stopped the run (a forced quit too: the file AutoSave last wrote
//!   must open in the window as it is on disk), counting only the editing time up to the last
//!   checkpoint.
//! - `fonts` (stab-003): Thai text selected, the Home tab's font list opened with the mouse,
//!   scrolled through every installed font (each drawn in its own face) and back, and a font
//!   picked from it; the window must answer within a second throughout and the text take the font.
//!   It clicks where a 1440×900 window draws the font box.
//! - `save` (stab-004): run as an ordinary user (not root, not an administrator), typed text
//!   lands in the document and saves into the user's own folder.
//! - `typing` (perf-005): a document of about 500 pages; each typed character shows within
//!   50 ms.
//! - `scroll` (perf-006): the same document scrolled by wheel turns at 30 frames per second or
//!   more.
//! - `touch` (hw-008): a finger drag scrolls, two fingers pinch to zoom, a tap places the caret.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

/// The checks, for the usage text.
const CHECKS: &str = "soak, fonts, save, typing, scroll, touch";

struct Opts {
    app: PathBuf,
    cli: PathBuf,
    out: PathBuf,
    hours: f64,
    checkpoint_min: f64,
    port: u16,
    resume: bool,
}

pub fn run(root: &Path, args: &[&str]) -> Result<(), String> {
    let mut it = args.iter();
    let check = it.next().copied().ok_or_else(|| format!("window: name a check ({CHECKS})"))?;
    let mut o = Opts {
        app: root.join("target/release/doc4me"),
        cli: root.join("target/release/doc4me-cli"),
        out: root.join("target/window-checks"),
        hours: 8.0,
        checkpoint_min: 15.0,
        port: 7990,
        resume: false,
    };
    while let Some(a) = it.next() {
        let mut val = || it.next().copied().ok_or_else(|| format!("{a} needs a value"));
        match *a {
            "--app" => o.app = PathBuf::from(val()?),
            "--cli" => o.cli = PathBuf::from(val()?),
            "--out" => o.out = PathBuf::from(val()?),
            "--hours" => o.hours = val()?.parse().map_err(|_| "--hours needs a number")?,
            "--checkpoint" => o.checkpoint_min = val()?.parse().map_err(|_| "--checkpoint needs minutes")?,
            "--port" => o.port = val()?.parse().map_err(|_| "--port needs a number")?,
            "--resume" => o.resume = true,
            other => return Err(format!("window: unknown option {other}")),
        }
    }
    std::fs::create_dir_all(&o.out).map_err(|e| format!("{}: {e}", o.out.display()))?;
    let report = match check {
        "soak" => soak(&o)?,
        "fonts" => fonts(&o)?,
        "save" => save(&o)?,
        "typing" => typing(&o)?,
        "scroll" => scroll(&o)?,
        "touch" => touch(&o)?,
        other => return Err(format!("window: unknown check `{other}` ({CHECKS})")),
    };
    let path = o.out.join(format!("{check}.json"));
    let text = serde_json::to_string_pretty(&report).unwrap_or_default();
    std::fs::write(&path, &text).map_err(|e| format!("{}: {e}", path.display()))?;
    println!("{text}");
    if report["pass"] == json!(true) { Ok(()) } else { Err(format!("window {check}: failed (see {})", path.display())) }
}

// ---------------------------------------------------------------------------------------------
// The app, driven through its control channel

struct App {
    child: Child,
    rd: BufReader<TcpStream>,
    wr: TcpStream,
    next: u64,
}

impl App {
    /// Starts the app with its own settings folder `home` (fresh preferences, its own control
    /// key) and connects. `files` are opened at start.
    fn start(o: &Opts, home: &Path, files: &[&Path]) -> Result<App, String> {
        let config = home.join("config");
        std::fs::create_dir_all(&config).map_err(|e| e.to_string())?;
        let key_path = config.join("doc4me").join(format!("control-key.local-{}", o.port));
        let _ = std::fs::remove_file(&key_path);
        let log = std::fs::File::create(home.join("app.log")).map_err(|e| e.to_string())?;
        let child = Command::new(&o.app)
            .arg("--control")
            .arg(o.port.to_string())
            .args(files)
            .env("XDG_CONFIG_HOME", &config)
            .env_remove("FLATPAK_ID")
            .stdout(Stdio::null())
            .stderr(log)
            .spawn()
            .map_err(|e| format!("{}: {e}", o.app.display()))?;
        let t0 = Instant::now();
        let key = loop {
            if let Ok(k) = std::fs::read_to_string(&key_path)
                && !k.trim().is_empty()
            {
                break k.trim().to_string();
            }
            if t0.elapsed() > Duration::from_secs(120) {
                return Err(format!("the app wrote no control key in 2 minutes ({})", home.join("app.log").display()));
            }
            std::thread::sleep(Duration::from_millis(200));
        };
        let wr = TcpStream::connect(("127.0.0.1", o.port)).map_err(|e| format!("connect: {e}"))?;
        wr.set_read_timeout(Some(Duration::from_secs(60))).map_err(|e| e.to_string())?;
        let rd = BufReader::new(wr.try_clone().map_err(|e| e.to_string())?);
        let mut app = App { child, rd, wr, next: 1 };
        app.call("auth", json!({"key": key}))?;
        // The first frames install the fonts; wait until the window lays out.
        while app.call("ui.inspect", json!({}))?["perf"]["frames"].as_u64().unwrap_or(0) < 3 {
            std::thread::sleep(Duration::from_millis(100));
        }
        // Click into the first page, as a user does before typing: the document takes the keys.
        app.call("ui.clickText", json!({"page": 0, "x": 72, "y": 72}))?;
        app.settle(4)?;
        Ok(app)
    }

    /// One request; its result, or the error the app replied with.
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next;
        self.next += 1;
        let line = json!({"id": id, "method": method, "params": params}).to_string();
        writeln!(self.wr, "{line}").map_err(|e| format!("{method}: {e}"))?;
        let mut reply = String::new();
        self.rd.read_line(&mut reply).map_err(|e| format!("{method}: no reply ({e}): the window hung or quit"))?;
        if reply.is_empty() {
            return Err(format!("{method}: the app closed the connection"));
        }
        let v: Value = serde_json::from_str(&reply).map_err(|e| format!("{method}: {e}"))?;
        if v["ok"] == json!(true) { Ok(v["result"].clone()) } else { Err(format!("{method}: {}", v["error"])) }
    }

    fn exec(&mut self, command: &str, params: Value) -> Result<Value, String> {
        self.call("engine.execute", json!({"command": command, "params": params}))
    }

    fn key(&mut self, key: &str, mods: Value) -> Result<(), String> {
        let mut p = mods;
        p["key"] = json!(key);
        self.call("ui.key", p).map(|_| ())
    }

    fn text(&mut self) -> Result<String, String> {
        Ok(self.exec("document.text", json!({}))?["text"].as_str().unwrap_or("").to_string())
    }

    fn frames(&mut self) -> Result<u64, String> {
        Ok(self.call("ui.inspect", json!({}))?["perf"]["frames"].as_u64().unwrap_or(0))
    }

    /// Waits until the window has laid out `n` more frames (the input sent so far is handled).
    fn settle(&mut self, n: u64) -> Result<(), String> {
        let start = self.frames()?;
        let t0 = Instant::now();
        while self.frames()? < start + n {
            if t0.elapsed() > Duration::from_secs(30) {
                return Err("the window stopped drawing for 30 s".into());
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        Ok(())
    }

    fn alive(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    /// Memory the app holds (Linux: resident set, kB).
    fn rss_kb(&self) -> Option<u64> {
        let s = std::fs::read_to_string(format!("/proc/{}/status", self.child.id())).ok()?;
        s.lines().find_map(|l| l.strip_prefix("VmRSS:")).and_then(|v| v.split_whitespace().next()?.parse().ok())
    }

    fn quit(mut self) {
        let _ = self.call("app.quit", json!({}));
        let t0 = Instant::now();
        while self.alive() && t0.elapsed() < Duration::from_secs(10) {
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }

    /// A forced quit: the process is killed, nothing is asked or saved.
    fn kill(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A fresh folder for one run of a check.
fn workdir(o: &Opts, name: &str) -> Result<PathBuf, String> {
    let d = o.out.join(name);
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    Ok(d)
}

/// The plain text of a file on disk, read by doc4me-cli (a separate program from the window).
fn file_text(o: &Opts, path: &Path) -> Result<String, String> {
    let out = Command::new(&o.cli).arg("text").arg(path).output().map_err(|e| format!("{}: {e}", o.cli.display()))?;
    if !out.status.success() {
        return Err(format!("doc4me-cli text {}: {}", path.display(), String::from_utf8_lossy(&out.stderr)));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Text compared the way both sides write it: line ends and trailing blank lines aside.
fn same_text(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.replace("\r\n", "\n").trim_end().to_string();
    norm(a) == norm(b)
}

/// A small, fixed pseudo-random sequence (no dependency, the same run every time).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
}

const WORDS: [&str; 24] = [
    "kiln",
    "glaze",
    "clay",
    "fire",
    "the",
    "potter",
    "shapes",
    "a",
    "bowl",
    "slowly",
    "under",
    "light",
    "ภาษาไทย",
    "แก้ว",
    "เตา",
    "and",
    "water",
    "cools",
    "every",
    "stone",
    "Año",
    "naïve",
    "über",
    "日本語",
];

fn sentence(r: &mut Rng) -> String {
    let n = 4 + r.below(10) as usize;
    let mut s: Vec<&str> = (0..n).map(|_| WORDS[r.below(WORDS.len() as u64) as usize]).collect();
    if let Some(first) = s.first_mut()
        && *first == "a"
    {
        *first = "A";
    }
    format!("{}. ", s.join(" "))
}

// ---------------------------------------------------------------------------------------------
// stab-001: AutoSave soak

fn soak(o: &Opts) -> Result<Value, String> {
    let dir = o.out.join("soak");
    let state_path = dir.join("state.json");
    let file = dir.join("soak.docx");
    // Progress so far: editing seconds, actions, checkpoints, mismatches, the random sequence,
    // the first memory reading and the resumes after a stopped run.
    let mut st = if o.resume {
        let text = std::fs::read_to_string(&state_path).map_err(|e| format!("{}: {e} (nothing to resume)", state_path.display()))?;
        serde_json::from_str::<Value>(&text).map_err(|e| format!("{}: {e}", state_path.display()))?
    } else {
        json!({"seconds": 0.0, "actions": 0, "checkpoints": [], "mismatches": 0, "rng": 0x5eed_d0c4_2026u64, "rssFirst": null, "resumes": []})
    };
    let mut app = if o.resume {
        // The run stopped with no warning, a forced quit: the window must open what AutoSave last
        // wrote, as it is on disk.
        let saved = file_text(o, &file)?;
        let mut app = App::start(o, &dir, &[&file])?;
        let reopened = app.text()?;
        let ok = same_text(&saved, &reopened) && !saved.trim().is_empty();
        let r = json!({"afterMinute": st["seconds"].as_f64().unwrap_or(0.0) / 60.0, "chars": reopened.chars().count(), "reopenMatchesFile": ok});
        println!("{}", json!({"resume": r}));
        if let Some(v) = st["resumes"].as_array_mut() {
            v.push(r);
        }
        if !ok {
            st["mismatches"] = json!(st["mismatches"].as_u64().unwrap_or(0) + 1);
        }
        app.exec("caret.docEnd", json!({}))?;
        // A file the user only opened isn't rewritten until they save it; then AutoSave on.
        app.exec("file.save", json!({}))?;
        app.exec("file.autosave", json!({"value": true}))?;
        app
    } else {
        let dir = workdir(o, "soak")?;
        let mut app = App::start(o, &dir, &[])?;
        // Saved once by the user (Save As), so AutoSave covers it, then AutoSave on.
        app.exec("file.saveAs", json!({"path": file}))?;
        app.exec("file.autosave", json!({"value": true}))?;
        app
    };
    let base = st["seconds"].as_f64().unwrap_or(0.0);
    let started = Instant::now();
    let elapsed = |started: &Instant| base + started.elapsed().as_secs_f64();
    let deadline = o.hours * 3600.0;
    let every = o.checkpoint_min * 60.0;
    let mut r = Rng(st["rng"].as_u64().filter(|n| *n != 0).unwrap_or(0x5eed_d0c4_2026));
    let mut actions = st["actions"].as_u64().unwrap_or(0);
    let mut mismatches = st["mismatches"].as_u64().unwrap_or(0);
    let mut checkpoints: Vec<Value> = st["checkpoints"].as_array().cloned().unwrap_or_default();
    let mut rss_first = st["rssFirst"].as_u64();
    let mut next_check = ((base / every).floor() + 1.0) * every;
    while elapsed(&started) < deadline {
        if !app.alive() {
            return Ok(json!({"pass": false, "why": "the app quit or crashed", "afterSeconds": elapsed(&started), "actions": actions}));
        }
        soak_action(&mut app, &mut r)?;
        actions += 1;
        std::thread::sleep(Duration::from_millis(150 + r.below(1200)));
        if elapsed(&started) >= next_check {
            next_check += every;
            // Idle long enough for AutoSave (2.5 s after the last change, the caret still).
            std::thread::sleep(Duration::from_secs(8));
            let shown = app.text()?;
            let saved = file_text(o, &file)?;
            let ok = same_text(&shown, &saved);
            mismatches += u64::from(!ok);
            let rss = app.rss_kb();
            rss_first = rss_first.or(rss);
            let now = elapsed(&started);
            let c = json!({"minute": (now / 60.0) as u64, "actions": actions, "chars": shown.chars().count(), "savedMatches": ok, "rssKb": rss});
            println!("{c}");
            checkpoints.push(c);
            st["seconds"] = json!(now);
            st["actions"] = json!(actions);
            st["mismatches"] = json!(mismatches);
            st["checkpoints"] = json!(checkpoints);
            st["rng"] = json!(r.0);
            st["rssFirst"] = json!(rss_first);
            let text = serde_json::to_string_pretty(&st).unwrap_or_default();
            std::fs::write(&state_path, text).map_err(|e| format!("{}: {e}", state_path.display()))?;
        }
    }
    // The last edits, AutoSave's moment, then a forced quit.
    for _ in 0..5 {
        soak_action(&mut app, &mut r)?;
    }
    std::thread::sleep(Duration::from_secs(8));
    let before_kill = app.text()?;
    let rss_last = app.rss_kb();
    app.kill();
    let after = file_text(o, &file)?;
    // And the window opens it with everything.
    let mut again = App::start(o, &dir, &[&file])?;
    let reopened = again.text()?;
    again.quit();
    let pass = mismatches == 0 && same_text(&before_kill, &after) && same_text(&before_kill, &reopened);
    Ok(json!({
        "pass": pass,
        "row": "stab-001",
        "hours": elapsed(&started) / 3600.0,
        "actions": actions,
        "checkpoints": checkpoints.len(),
        "checkpointMismatches": mismatches,
        "resumes": st["resumes"],
        "charsAtKill": before_kill.chars().count(),
        "fileMatchesAfterKill": same_text(&before_kill, &after),
        "reopenMatches": same_text(&before_kill, &reopened),
        "rssKb": [rss_first, rss_last],
        "log": checkpoints,
    }))
}

/// One editing step, as a user makes them: typing, Enter, deleting, formatting, a table, undo.
/// The document is kept to a few dozen pages by deleting from its start.
fn soak_action(app: &mut App, r: &mut Rng) -> Result<(), String> {
    let none = json!({});
    match r.below(20) {
        0..=8 => {
            let s = sentence(r);
            app.call("ui.text", json!({"text": s}))?;
        }
        9 | 10 => app.key("Enter", none)?,
        11 => {
            for _ in 0..1 + r.below(6) {
                app.key("Backspace", json!({}))?;
            }
        }
        12 => {
            // Select the last few characters and make them bold or italic.
            for _ in 0..1 + r.below(8) {
                app.key("Left", json!({"shift": true}))?;
            }
            app.key(if r.below(2) == 0 { "B" } else { "I" }, json!({"cmd": true}))?;
            app.key("End", json!({}))?;
        }
        13 => {
            app.exec("insert.table", json!({"rows": 2, "cols": 2}))?;
            app.call("ui.text", json!({"text": "cell"}))?;
            app.exec("caret.docEnd", json!({}))?;
        }
        14 => app.key("Z", json!({"cmd": true}))?,
        15 => app.key("Y", json!({"cmd": true}))?,
        16 => {
            app.exec("caret.docEnd", json!({}))?;
        }
        17 => {
            app.key("Home", json!({}))?;
            app.call("ui.text", json!({"text": "Note: "}))?;
            app.key("End", json!({}))?;
        }
        _ => {
            let words = app.text()?.split_whitespace().count();
            if words > 12_000 {
                // Trim the start: select from the top down and delete.
                app.exec("caret.docStart", json!({}))?;
                for _ in 0..40 {
                    app.key("Down", json!({"shift": true}))?;
                }
                app.key("Delete", json!({}))?;
                app.exec("caret.docEnd", json!({}))?;
            } else {
                app.call("ui.text", json!({"text": sentence(r)}))?;
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// stab-003: the font list with Thai text selected

fn fonts(o: &Opts) -> Result<Value, String> {
    let dir = workdir(o, "fonts")?;
    let mut app = App::start(o, &dir, &[])?;
    app.call("ui.text", json!({"text": "สวัสดีครับ ภาษาไทยกับแบบอักษร"}))?;
    app.settle(2)?;
    app.exec("select.all", json!({}))?;
    // The installed families (fontconfig's list where there is one).
    let n = Command::new("fc-list")
        .args([":", "family"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().flat_map(|l| l.split(',')).collect::<std::collections::BTreeSet<_>>().len())
        .unwrap_or(0);
    // The Home tab's font box, opened with the mouse (its arrow, where a 1440×900 window draws it).
    let mut slowest = 0f64;
    let mut timed = |app: &mut App, f: &mut dyn FnMut(&mut App) -> Result<(), String>| -> Result<(), String> {
        let t = Instant::now();
        f(app)?;
        app.settle(1)?;
        slowest = slowest.max(t.elapsed().as_secs_f64() * 1000.0);
        Ok(())
    };
    let window = app.call("ui.inspect", json!({}))?["window"].clone();
    if window != json!([1440.0, 900.0]) {
        return Err(format!("window fonts: the window is {window}, not 1440×900 (run under xvfb-run -s \"-screen 0 1440x900x24\")"));
    }
    timed(&mut app, &mut |a| a.call("ui.click", json!({"x": 280, "y": 84})).map(|_| ()))?;
    // Through the whole list and back, every font drawn in its own face on the way.
    let pages = (n / 10).max(30);
    for _ in 0..pages {
        timed(&mut app, &mut |a| a.call("ui.scroll", json!({"x": 200, "y": 330, "dy": -240.0})).map(|_| ()))?;
    }
    for _ in 0..pages {
        timed(&mut app, &mut |a| a.call("ui.scroll", json!({"x": 200, "y": 330, "dy": 240.0})).map(|_| ()))?;
    }
    // Pick the fourth entry.
    timed(&mut app, &mut |a| a.call("ui.click", json!({"x": 200, "y": 193})).map(|_| ()))?;
    app.settle(2)?;
    let state = app.exec("format.state", json!({}))?;
    let font = state["font"].as_str().unwrap_or("").to_string();
    let text = app.text()?;
    let alive = app.alive();
    app.quit();
    let pass = alive && slowest < 1000.0 && font != "Sarabun" && !font.is_empty() && text.contains("สวัสดีครับ");
    Ok(
        json!({"pass": pass, "row": "stab-003", "installedFamilies": n, "slowestStepMs": slowest, "fontBefore": "Sarabun", "fontAfter": font, "textKept": text.contains("สวัสดีครับ")}),
    )
}

// ---------------------------------------------------------------------------------------------
// stab-004: typing and saving as an ordinary user

fn save(o: &Opts) -> Result<Value, String> {
    #[cfg(unix)]
    let uid = {
        let out = Command::new("id").arg("-u").output().map_err(|e| e.to_string())?;
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };
    #[cfg(not(unix))]
    let uid = String::from("?");
    if uid == "0" {
        return Err("window save: run this check as an ordinary user, not root".into());
    }
    let dir = workdir(o, "save")?;
    let docs = dir.join("Documents");
    std::fs::create_dir_all(&docs).map_err(|e| e.to_string())?;
    let app_dir_writable = o.app.parent().is_some_and(writable);
    let mut app = App::start(o, &dir, &[])?;
    let typed = "Typed as an ordinary user. ภาษาไทยก็พิมพ์ได้";
    app.call("ui.text", json!({"text": typed}))?;
    app.settle(2)?;
    let shown = app.text()?;
    let file = docs.join("user.docx");
    app.exec("file.saveAs", json!({"path": file}))?;
    app.call("ui.text", json!({"text": " More."}))?;
    app.settle(2)?;
    app.key("S", json!({"cmd": true}))?;
    app.settle(3)?;
    let shown_after = app.text()?;
    app.quit();
    let saved = file_text(o, &file)?;
    let pass = same_text(&shown, typed) && same_text(&saved, &shown_after) && shown_after.ends_with("More.");
    Ok(
        json!({"pass": pass, "row": "stab-004", "uid": uid, "appFolderWritable": app_dir_writable, "typedShown": same_text(&shown, typed), "savedMatches": same_text(&saved, &shown_after)}),
    )
}

fn writable(d: &Path) -> bool {
    let probe = d.join(".doc4me-write-probe");
    let ok = std::fs::write(&probe, b"x").is_ok();
    let _ = std::fs::remove_file(&probe);
    ok
}

// ---------------------------------------------------------------------------------------------
// perf-005 / perf-006: typing and scrolling in about 500 pages

/// About 500 pages of English text: performance.tsv's generated document of 1,600 paragraphs.
fn big_document(o: &Opts, dir: &Path) -> Result<PathBuf, String> {
    let src = dir.join("big.txt");
    std::fs::write(&src, crate::readiness::sample_text(1600)).map_err(|e| e.to_string())?;
    let out = dir.join("big.docx");
    let st = Command::new(&o.cli).arg("convert").arg(&src).arg(&out).status().map_err(|e| e.to_string())?;
    if !st.success() {
        return Err("doc4me-cli convert big.txt failed".into());
    }
    Ok(out)
}

fn pages(app: &mut App) -> Result<u64, String> {
    let v = app.exec("document.layout", json!({}))?;
    Ok(v["pages"].as_u64().or_else(|| v["pages"].as_array().map(|a| a.len() as u64)).unwrap_or(0))
}

fn percentile(v: &mut [f64], p: f64) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v.get(((v.len() as f64 - 1.0) * p).round() as usize).copied().unwrap_or(f64::NAN)
}

fn typing(o: &Opts) -> Result<Value, String> {
    let dir = workdir(o, "typing")?;
    let file = big_document(o, &dir)?;
    let mut app = App::start(o, &dir, &[&file])?;
    let n_pages = pages(&mut app)?;
    // In the middle of the document.
    app.exec("edit.goto", json!({"page": n_pages / 2})).ok();
    app.settle(3)?;
    // Each character: the app's time over the frame that read it, from the frame's start to its
    // drawing done (what the graphics card then shows is not included), and, for the record,
    // the time from sending it to the next frame after that one, which includes the control
    // channel's round trips and, without a graphics card, the processor drawing the window.
    let perf = |app: &mut App| app.call("ui.inspect", json!({})).map(|v| v["perf"].clone());
    let (mut ms, mut wall) = (Vec::new(), Vec::new());
    for (i, c) in "The quick brown fox jumps over the lazy dog again and again ".chars().enumerate() {
        let before = perf(&mut app)?["keyFrames"].as_u64().unwrap_or(0);
        let t = Instant::now();
        app.call("ui.text", json!({"text": c.to_string()}))?;
        let p = loop {
            let p = perf(&mut app)?;
            if p["keyFrames"].as_u64().unwrap_or(0) > before {
                break p;
            }
            if t.elapsed() > Duration::from_secs(10) {
                return Err("the window stopped drawing while typing".into());
            }
        };
        // The first characters warm the caches.
        if i >= 3 {
            ms.push(p["keyFrameMs"].as_f64().unwrap_or(f64::NAN));
            wall.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        if std::env::var_os("DOC4ME_WINDOW_VERBOSE").is_some() {
            println!("{c:?} {:.1} ms {p}", t.elapsed().as_secs_f64() * 1000.0);
        }
    }
    let typed = app.text()?.contains("The quick brown fox jumps over the lazy dog again and again");
    app.quit();
    let (p50, p95, max) = (percentile(&mut ms, 0.5), percentile(&mut ms, 0.95), percentile(&mut ms, 1.0));
    // The screen shows a drawn frame at its next refresh: one 60 Hz refresh is allowed for that.
    const REFRESH_MS: f64 = 1000.0 / 60.0;
    let pass = typed && n_pages >= 400 && p95 + REFRESH_MS <= 50.0;
    Ok(json!({
        "pass": pass, "row": "perf-005", "pages": n_pages, "chars": ms.len(), "msP50": p50, "msP95": p95, "msMax": max, "typed": typed,
        "refreshAllowanceMs": REFRESH_MS, "shownMsP95": p95 + REFRESH_MS, "roundTripMsP50": percentile(&mut wall, 0.5),
    }))
}

fn scroll(o: &Opts) -> Result<Value, String> {
    let dir = workdir(o, "scroll")?;
    let file = big_document(o, &dir)?;
    let mut app = App::start(o, &dir, &[&file])?;
    let n_pages = pages(&mut app)?;
    app.settle(3)?;
    let top = app.call("ui.inspect", json!({}))?["pages"].clone();
    // Wheel turns down the document, one per frame, for about five seconds' worth.
    let f0 = app.frames()?;
    let t = Instant::now();
    let turns = 300;
    // The app's own time over each frame drawn while scrolling (the graphics card's not included).
    let mut cpu = Vec::new();
    for _ in 0..turns {
        let p = app.call("ui.inspect", json!({}))?["perf"].clone();
        let before = p["frames"].as_u64().unwrap_or(0);
        cpu.push(p["frameCpuMs"].as_f64().unwrap_or(f64::NAN));
        app.call("ui.scroll", json!({"dy": -120.0}))?;
        while app.frames()? < before + 1 {
            if t.elapsed() > Duration::from_secs(300) {
                return Err("the window stopped drawing while scrolling".into());
            }
        }
    }
    let secs = t.elapsed().as_secs_f64();
    let frames = app.frames()? - f0;
    let moved = app.call("ui.inspect", json!({}))?["pages"] != top;
    app.quit();
    let fps = frames as f64 / secs;
    let pass = moved && n_pages >= 400 && fps >= 30.0;
    Ok(json!({
        "pass": pass, "row": "perf-006", "pages": n_pages, "wheelTurns": turns, "frames": frames, "seconds": secs, "fps": fps, "moved": moved,
        "frameCpuMsP50": percentile(&mut cpu.clone(), 0.5), "frameCpuMsP95": percentile(&mut cpu, 0.95),
    }))
}

// ---------------------------------------------------------------------------------------------
// hw-008: touch

fn touch(o: &Opts) -> Result<Value, String> {
    let dir = workdir(o, "touch")?;
    let file = big_document(o, &dir)?;
    let mut app = App::start(o, &dir, &[&file])?;
    app.settle(3)?;
    let inspect = |app: &mut App| app.call("ui.inspect", json!({}));
    let v = inspect(&mut app)?;
    let c = v["canvasRect"].as_array().cloned().unwrap_or_default();
    let num = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0);
    let (cx, cy) = (num(0) + num(2) / 2.0, num(1) + num(3) / 2.0);
    let first_page_top = |v: &Value| v["pages"][0][1].as_f64().unwrap_or(0.0);
    let top0 = first_page_top(&v);
    // One finger drags up: the document scrolls down.
    let t = |app: &mut App, phase: &str, id: u64, x: f64, y: f64| app.call("ui.touch", json!({"phase": phase, "id": id, "x": x, "y": y})).map(|_| ());
    t(&mut app, "start", 0, cx, cy + 150.0)?;
    app.settle(1)?;
    for k in 1..=10 {
        t(&mut app, "move", 0, cx, cy + 150.0 - 30.0 * k as f64)?;
        app.settle(1)?;
    }
    t(&mut app, "end", 0, cx, cy - 150.0)?;
    app.settle(2)?;
    let v = inspect(&mut app)?;
    let scrolled = first_page_top(&v) < top0 - 100.0;
    let sel_after_drag = app.exec("document.selection", json!({}))?;
    // Two fingers spread apart: zoom in.
    let scale0 = v["scale"].as_f64().unwrap_or(1.0);
    t(&mut app, "start", 1, cx - 40.0, cy)?;
    t(&mut app, "start", 2, cx + 40.0, cy)?;
    app.settle(1)?;
    for k in 1..=10 {
        let d = 40.0 + 12.0 * k as f64;
        t(&mut app, "move", 1, cx - d, cy)?;
        t(&mut app, "move", 2, cx + d, cy)?;
        app.settle(1)?;
    }
    t(&mut app, "end", 1, cx - 160.0, cy)?;
    t(&mut app, "end", 2, cx + 160.0, cy)?;
    app.settle(2)?;
    let scale1 = inspect(&mut app)?["scale"].as_f64().unwrap_or(1.0);
    let zoomed = scale1 > scale0 * 1.2;
    // A tap places the caret where it lands.
    let caret0 = app.exec("document.selection", json!({}))?;
    t(&mut app, "start", 3, cx, cy - 60.0)?;
    app.settle(1)?;
    t(&mut app, "end", 3, cx, cy - 60.0)?;
    app.settle(2)?;
    let caret1 = app.exec("document.selection", json!({}))?;
    let tapped = caret1 != caret0;
    app.quit();
    let pass = scrolled && zoomed && tapped;
    Ok(
        json!({"pass": pass, "row": "hw-008", "scrolled": scrolled, "zoomed": zoomed, "scale": [scale0, scale1], "tapPlacedCaret": tapped, "selectionAfterDrag": sel_after_drag}),
    )
}
