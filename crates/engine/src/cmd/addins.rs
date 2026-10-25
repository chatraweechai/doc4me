//! Insert › Add-ins › Get Add-ins.
//!
//! Word's add-ins come from Microsoft's Office Store and run on Office.js. Doc4Me's add-ins are
//! packs of macros (named lists of Doc4Me commands, as View › Macros records them), listed in a
//! free catalog kept in Doc4Me's own repository (`addins/catalog.json`, served by GitHub) with
//! each pack's SHA-256. The front end downloads the catalog and a pack; this module checks the
//! pack and installs its macros, so an agent or script can install one too.
//!
//! An add-in only edits the document: a pack is refused when a step uses a command outside the
//! document (files, the network, the app's settings, other macros) or passes a path, data, an
//! address or a password, and every step must be a command this version has. Packs are small
//! and their names short, so a hostile catalog can't flood the interface.

use serde_json::{Value, json};
use sha2::Digest;

use crate::{CmdError, CmdResult, CommandSpec, Registry, Session, p};

/// The catalog's address, and the folder its packs are in.
pub const CATALOG_URL: &str = "https://raw.githubusercontent.com/chatraweechai/doc4me/main/addins/catalog.json";
pub const PACK_BASE: &str = "https://raw.githubusercontent.com/chatraweechai/doc4me/main/addins/";

/// Largest catalog and pack, in bytes; most entries, macros and steps.
pub const MAX_CATALOG_BYTES: u64 = 256 << 10;
pub const MAX_PACK_BYTES: u64 = 256 << 10;
const MAX_ENTRIES: usize = 200;
const MAX_MACROS: usize = 50;
const MAX_STEPS: usize = 500;

/// Command families an add-in may not use: files, printing and the app (`file.`), templates,
/// macros and options (`tools.`), the interface (`ui.`), add-ins themselves, mail merge data
/// sources, Zotero, and what asks the system or the network for something.
const DENIED_PREFIXES: &[&str] = &["file.", "tools.", "ui.", "addins.", "mailings.", "zotero.", "font.", "readAloud."];
const DENIED: &[&str] = &[
    "insert.textFromFile",
    "insert.screenshot",
    "insert.onlinePicture",
    "insert.video",
    "insert.model3d",
    "picture.playVideo",
    "review.translate",
    "review.translateDocument",
    "edit.paste",
    "edit.pasteSpecial",
];
/// Parameter names an add-in may not pass, at any depth.
const DENIED_PARAMS: &[&str] = &["path", "data", "url", "password", "file", "template"];

/// A catalog entry.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub version: String,
    /// The pack's file name in [`PACK_BASE`], and its SHA-256 (lower-case hex).
    pub file: String,
    pub sha256: String,
}

fn text(v: &Value, k: &str, max: usize) -> String {
    v.get(k).and_then(Value::as_str).unwrap_or("").trim().chars().filter(|c| !c.is_control()).take(max).collect()
}

fn valid_id(id: &str) -> bool {
    (1..=40).contains(&id.len()) && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// The entries of a catalog; malformed entries are skipped.
pub fn parse_catalog(bytes: &[u8]) -> Result<Vec<Entry>, String> {
    let v: Value = serde_json::from_slice(bytes).map_err(|_| "the add-in catalog can't be read".to_string())?;
    let list = v.get("addins").and_then(Value::as_array).ok_or("the add-in catalog lists no add-ins")?;
    Ok(list
        .iter()
        .take(MAX_ENTRIES)
        .filter_map(|e| {
            let entry = Entry {
                id: text(e, "id", 40),
                name: text(e, "name", 80),
                description: text(e, "description", 400),
                author: text(e, "author", 80),
                license: text(e, "license", 60),
                version: text(e, "version", 20),
                file: text(e, "file", 60),
                sha256: text(e, "sha256", 64).to_ascii_lowercase(),
            };
            let file_ok = entry.file.strip_suffix(".json").is_some_and(valid_id);
            let hash_ok = entry.sha256.len() == 64 && entry.sha256.bytes().all(|b| b.is_ascii_hexdigit());
            (valid_id(&entry.id) && !entry.name.is_empty() && file_ok && hash_ok).then_some(entry)
        })
        .collect())
}

/// A checked pack: its id and name, and its macros as (name, steps).
#[derive(Clone, Debug)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub macros: Vec<(String, Vec<(String, Value)>)>,
}

/// Whether `v` names a denied parameter anywhere in it.
fn has_denied_param(v: &Value, depth: usize) -> bool {
    match v {
        _ if depth > 16 => true,
        Value::Object(m) => m.iter().any(|(k, v)| DENIED_PARAMS.iter().any(|d| k.eq_ignore_ascii_case(d)) || has_denied_param(v, depth + 1)),
        Value::Array(a) => a.iter().any(|v| has_denied_param(v, depth + 1)),
        _ => false,
    }
}

/// Check a pack's JSON text: its size, its SHA-256 when one is given, and every step.
pub fn check_pack(json_text: &str, sha256: Option<&str>, reg: &Registry) -> Result<Pack, String> {
    if json_text.len() as u64 > MAX_PACK_BYTES {
        return Err("the add-in is too large".into());
    }
    if let Some(want) = sha256 {
        let got: String = sha2::Sha256::digest(json_text.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
        if !got.eq_ignore_ascii_case(want.trim()) {
            return Err("the add-in doesn't match its catalog entry (SHA-256)".into());
        }
    }
    let v: Value = serde_json::from_str(json_text).map_err(|_| "the add-in can't be read".to_string())?;
    let id = text(&v, "id", 40);
    if !valid_id(&id) {
        return Err("the add-in has no valid id".into());
    }
    let name = text(&v, "name", 80);
    let list = v.get("macros").and_then(Value::as_array).filter(|m| !m.is_empty()).ok_or("the add-in has no macros")?;
    if list.len() > MAX_MACROS {
        return Err("the add-in has too many macros".into());
    }
    let mut macros = Vec::new();
    for m in list {
        let mname = text(m, "name", 80);
        if mname.is_empty() {
            return Err("an add-in macro has no name".into());
        }
        let steps = m
            .get("steps")
            .and_then(Value::as_array)
            .filter(|s| !s.is_empty() && s.len() <= MAX_STEPS)
            .ok_or_else(|| format!("macro {mname:?} has no steps or too many"))?;
        let mut out = Vec::new();
        for st in steps {
            let cmd = st.get("command").and_then(Value::as_str).unwrap_or("");
            let params = st.get("params").cloned().unwrap_or(json!({}));
            if reg.get(cmd).is_none() {
                return Err(format!("macro {mname:?} uses {cmd:?}, which this version of Doc4Me doesn't have"));
            }
            if DENIED_PREFIXES.iter().any(|p| cmd.starts_with(p)) || DENIED.contains(&cmd) {
                return Err(format!("macro {mname:?} uses {cmd:?}, which add-ins may not use"));
            }
            if !params.is_object() || has_denied_param(&params, 0) {
                return Err(format!("macro {mname:?} passes {cmd:?} a file, data or an address, which add-ins may not do"));
            }
            out.push((cmd.to_string(), params));
        }
        macros.push((mname, out));
    }
    Ok(Pack { name: if name.is_empty() { id.clone() } else { name }, id, macros })
}

/// The macro name an add-in's macro is installed under.
pub fn macro_key(id: &str, name: &str) -> String {
    format!("{id}.{name}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("addins.get", "Get Add-ins", "Insert › Add-ins", get)
            .params(r#"{"install"?: pack JSON text, "sha256"?: hex (checked against the text), "remove"?: id} → the installed add-ins' macros; no params shows Get Add-ins"#)
            .pure(),
    ]
}

fn get(s: &mut Session, v: &Value) -> CmdResult {
    let mut installed = None;
    if let Some(text) = v.get("install") {
        let text = match text {
            Value::String(t) => t.clone(),
            other => other.to_string(),
        };
        let pack = check_pack(&text, p::str(v, "sha256"), &s.registry.clone()).map_err(CmdError::Params)?;
        remove(s, &pack.id);
        for (name, steps) in &pack.macros {
            s.macros.insert(macro_key(&pack.id, name), steps.clone());
        }
        installed =
            Some(json!({"id": pack.id, "name": pack.name, "macros": pack.macros.iter().map(|(n, _)| macro_key(&pack.id, n)).collect::<Vec<_>>()}));
    } else if let Some(id) = p::str(v, "remove") {
        remove(s, id);
    } else {
        s.ui_requests.push(json!({"open": "getAddins"}));
    }
    Ok(json!({"installed": installed, "catalog": CATALOG_URL}))
}

fn remove(s: &mut Session, id: &str) {
    if valid_id(id) {
        let prefix = format!("{id}.");
        s.macros.retain(|k, _| !k.starts_with(&prefix));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> Session {
        Session::new(doc4me_doc::Document::from_text("one  two"))
    }

    #[test]
    fn a_pack_installs_runs_and_goes() {
        let mut s = session();
        let pack = r#"{"id": "tidy", "name": "Tidy", "macros": [{"name": "Single spaces", "steps": [{"command": "edit.replaceAll", "params": {"text": "  ", "with": " "}}]}]}"#;
        let r = s.run("addins.get", &json!({"install": pack})).unwrap();
        assert_eq!(r["installed"]["macros"][0], "tidy.Single spaces");
        s.run("tools.macros", &json!({"run": "tidy.Single spaces"})).unwrap();
        assert_eq!(s.doc.plain_text(doc4me_doc::StoryRef::Body), "one two");
        s.run("addins.get", &json!({"remove": "tidy"})).unwrap();
        assert!(s.macros.is_empty());
        // The catalog's hash must match.
        assert!(s.run("addins.get", &json!({"install": pack, "sha256": "00".repeat(32)})).is_err());
        let good: String = sha2::Sha256::digest(pack.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
        assert!(s.run("addins.get", &json!({"install": pack, "sha256": good})).is_ok());
        // No params: Get Add-ins.
        s.run("addins.get", &json!({})).unwrap();
        assert!(s.ui_requests.iter().any(|r| r["open"] == "getAddins"));
    }

    #[test]
    fn packs_that_reach_outside_the_document_are_refused() {
        let reg = crate::cmd::registry();
        let pack = |steps: &str| format!(r#"{{"id": "x", "macros": [{{"name": "m", "steps": [{steps}]}}]}}"#);
        for bad in [
            r#"{"command": "file.save", "params": {}}"#,
            r#"{"command": "tools.macros", "params": {"run": "m"}}"#,
            r#"{"command": "insert.picture", "params": {"path": "/etc/passwd"}}"#,
            r#"{"command": "insert.picture", "params": {"nested": [{"Data": "AAAA"}]}}"#,
            r#"{"command": "insert.textFromFile", "params": {}}"#,
            r#"{"command": "no.suchCommand", "params": {}}"#,
            r#"{"command": "format.bold", "params": 5}"#,
        ] {
            assert!(check_pack(&pack(bad), None, &reg).is_err(), "{bad}");
        }
        assert!(check_pack(&pack(r#"{"command": "format.bold"}"#), None, &reg).is_ok());
        assert!(check_pack(r#"{"id": "Bad Id!", "macros": [{"name": "m", "steps": [{"command": "format.bold"}]}]}"#, None, &reg).is_err());
        assert!(check_pack("not json", None, &reg).is_err());
        assert!(check_pack(&"x".repeat(MAX_PACK_BYTES as usize + 1), None, &reg).is_err());
    }

    #[test]
    fn catalog_entries_are_checked() {
        let cat = br#"{"addins": [
            {"id": "ok", "name": "OK", "file": "ok.json", "sha256": "ABCDEFabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123"},
            {"id": "bad", "name": "Bad file", "file": "../../etc/passwd", "sha256": "abcdefabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123"},
            {"id": "nohash", "name": "No hash", "file": "nohash.json", "sha256": "xyz"}
        ]}"#;
        let e = parse_catalog(cat).unwrap();
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].sha256, "abcdefabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123");
        assert!(parse_catalog(b"[]").is_err());
    }

    /// The add-ins in the repository: the catalog reads, every pack matches its hash and passes
    /// the checks with this version's commands, and runs on a sample document.
    #[test]
    fn the_repository_catalog_and_packs_are_valid() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../addins");
        let catalog = std::fs::read(dir.join("catalog.json")).unwrap();
        let entries = parse_catalog(&catalog).unwrap();
        assert!(entries.len() >= 3);
        let reg = crate::cmd::registry();
        for e in entries {
            let text = std::fs::read_to_string(dir.join(&e.file)).unwrap();
            let pack = check_pack(&text, Some(&e.sha256), &reg).unwrap_or_else(|err| panic!("{}: {err}", e.file));
            assert_eq!(pack.id, e.id);
            for (name, _) in &pack.macros {
                let mut s = Session::new(crate::sample::sample_document());
                s.run("addins.get", &json!({"install": text})).unwrap();
                s.run("tools.macros", &json!({"run": macro_key(&pack.id, name)})).unwrap_or_else(|err| panic!("{} › {name}: {err}", e.id));
            }
        }
    }
}
