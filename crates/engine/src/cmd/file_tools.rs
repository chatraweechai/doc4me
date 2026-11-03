//! File › Open's file tools from Word's classic dialogs: Copy File (`wdDialogCopyFile`) and
//! Find File / Search (`wdDialogFileFind`, `wdDialogSearch`): find documents in a folder by name
//! and by the text they hold. Every path goes through the automation folder checks
//! ([`crate::file_access`]), so an agent confined to its folders reaches nothing else.

use std::path::{Path, PathBuf};

use doc4me_doc::StoryRef;
use serde_json::{Value, json};

use crate::file_access::{check_read, check_write};
use crate::{CmdError, CmdResult, CommandSpec, Session, p};

/// Most files a search lists.
const MAX_FOUND: usize = 500;
/// Most folder entries a search looks at (a whole disk is not searched by accident).
const MAX_SCANNED: usize = 20_000;
/// Deepest folder a search with subfolders goes into.
const MAX_DEPTH: usize = 8;
/// Most documents a search by text opens and reads.
const MAX_OPENED: usize = 200;
/// Largest document a search by text reads, in bytes.
const MAX_READ_BYTES: u64 = 50 * 1024 * 1024;
/// Longest name pattern or text to find, in characters.
const MAX_PATTERN: usize = 255;
/// The documents a search finds when no name is given.
const DOCUMENT_EXTS: &[&str] = &["docx", "docm", "dotx", "dotm", "doc", "dot", "odt", "rtf", "txt", "md", "html", "htm"];

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("file.copyFile", "Copy File", "File › Open", copy_file)
            .params(r#"{"fileName": path, "directory": folder, "overwrite"?: bool (default false)} → {"path"}: a copy of the file in the folder, under the same name"#)
            .pure(),
        CommandSpec::new("file.findFiles", "Find Files", "File › Open", find_files)
            .params(r#"{"path": folder, "name"?: pattern (* and ?; default every document), "text"?: string (the document holds it, without regard to case), "subfolders"?: bool (default false)} → {"files": [{"path", "size", "modified" (seconds since 1970)}], "complete": bool (false when a limit stopped the search)}"#)
            .pure(),
    ]
}

fn copy_file(s: &mut Session, v: &Value) -> CmdResult {
    let _ = s;
    let from = PathBuf::from(p::req_str(v, "fileName")?);
    let dir = PathBuf::from(p::req_str(v, "directory")?);
    let overwrite = p::bool(v, "overwrite").unwrap_or(false);
    check_read(&from.to_string_lossy()).map_err(CmdError::Failed)?;
    check_write(&dir.to_string_lossy()).map_err(CmdError::Failed)?;
    if !from.is_file() {
        return Err(CmdError::Params(format!("`{}` is not a file", from.display())));
    }
    if !dir.is_dir() {
        return Err(CmdError::Params(format!("`{}` is not a folder", dir.display())));
    }
    let name = from.file_name().ok_or_else(|| CmdError::Params("`fileName` names no file".into()))?;
    let to = dir.join(name);
    check_write(&to.to_string_lossy()).map_err(CmdError::Failed)?;
    if same_file(&from, &to) {
        return Err(CmdError::Params("the file is already in that folder".into()));
    }
    if to.exists() && !overwrite {
        return Err(CmdError::Failed(format!("`{}` already exists (overwrite: true replaces it)", to.display())));
    }
    crate::io::copy_file(&from.to_string_lossy(), &to.to_string_lossy()).map_err(CmdError::Failed)?;
    Ok(json!({"path": to.to_string_lossy()}))
}

/// Are `a` and `b` one file (after links)?
fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

/// Does `name` match `pattern` (`*` any run of characters, `?` one), without regard to case?
fn matches_pattern(pattern: &[char], name: &[char]) -> bool {
    // Iterative wildcard match with backtracking to the last `*`.
    let (mut p, mut n) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while n < name.len() {
        if p < pattern.len() && (pattern[p] == '?' || pattern[p] == name[n]) {
            p += 1;
            n += 1;
        } else if p < pattern.len() && pattern[p] == '*' {
            star = Some((p, n));
            p += 1;
        } else if let Some((sp, sn)) = star {
            p = sp + 1;
            n = sn + 1;
            star = Some((sp, sn + 1));
        } else {
            return false;
        }
    }
    pattern[p..].iter().all(|c| *c == '*')
}

fn lower(t: &str) -> Vec<char> {
    t.chars().flat_map(char::to_lowercase).collect()
}

fn find_files(s: &mut Session, v: &Value) -> CmdResult {
    let _ = s;
    let root = PathBuf::from(p::req_str(v, "path")?);
    check_read(&root.to_string_lossy()).map_err(CmdError::Failed)?;
    if !root.is_dir() {
        return Err(CmdError::Params(format!("`{}` is not a folder", root.display())));
    }
    let pattern = match p::str(v, "name").map(str::trim).filter(|t| !t.is_empty()) {
        Some(t) if t.chars().count() > MAX_PATTERN => return Err(CmdError::Params(format!("`name` is at most {MAX_PATTERN} characters"))),
        Some(t) => Some(lower(t)),
        None => None,
    };
    let text = match p::str(v, "text").filter(|t| !t.trim().is_empty()) {
        Some(t) if t.chars().count() > MAX_PATTERN => return Err(CmdError::Params(format!("`text` is at most {MAX_PATTERN} characters"))),
        Some(t) => Some(t.to_lowercase()),
        None => None,
    };
    let subfolders = p::bool(v, "subfolders").unwrap_or(false);

    let mut found = Vec::new();
    let (mut scanned, mut opened) = (0usize, 0usize);
    let mut complete = true;
    // Folders still to look in, with their depth; read in name order so results are stable.
    let mut folders = vec![(root, 0usize)];
    'search: while let Some((dir, depth)) = folders.pop() {
        let Ok(entries) = crate::io::list_dir(&dir.to_string_lossy()) else { continue };
        let mut subdirs = Vec::new();
        for e in entries {
            scanned += 1;
            if scanned > MAX_SCANNED {
                complete = false;
                break 'search;
            }
            // Links are not followed into other folders: a search stays in the tree it was given.
            if e.is_dir {
                if subfolders && depth < MAX_DEPTH {
                    subdirs.push((e.path, depth + 1));
                } else if subfolders {
                    complete = false;
                }
                continue;
            }
            if !e.is_file || check_read(&e.path.to_string_lossy()).is_err() {
                continue;
            }
            let name = lower(&e.name);
            let wanted = match &pattern {
                Some(pat) => matches_pattern(pat, &name),
                None => {
                    let ext = e.path.extension().map(|x| x.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
                    DOCUMENT_EXTS.contains(&ext.as_str())
                }
            };
            if !wanted {
                continue;
            }
            if let Some(t) = &text {
                if opened >= MAX_OPENED {
                    complete = false;
                    break 'search;
                }
                if e.len > MAX_READ_BYTES {
                    continue;
                }
                opened += 1;
                let Ok((doc, _)) = crate::io::open_path_with(&e.path, None) else { continue };
                if !doc.plain_text(StoryRef::Body).to_lowercase().contains(t.as_str()) {
                    continue;
                }
            }
            found.push(json!({"path": e.path.to_string_lossy(), "size": e.len, "modified": e.modified}));
            if found.len() >= MAX_FOUND {
                complete = false;
                break 'search;
            }
        }
        // Deeper folders in name order: pushed last first, popped first first.
        folders.extend(subdirs.into_iter().rev());
    }
    Ok(json!({"files": found, "complete": complete}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &mut Session, id: &str, v: Value) -> Result<Value, CmdError> {
        s.run(id, &v)
    }

    #[test]
    fn wildcards_match_names_without_regard_to_case() {
        let m = |p: &str, n: &str| matches_pattern(&lower(p), &lower(n));
        assert!(m("*.docx", "Report.DOCX"));
        assert!(m("re?ort*", "report 2.txt"));
        assert!(!m("*.doc", "a.docx"));
        assert!(m("*", ""));
        assert!(m("a*b*c", "aXXbYYc") && !m("a*b*c", "aXXbYY"));
    }

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("doc4me-file-tools-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn copy_file_copies_into_a_folder_and_keeps_what_is_there() {
        let d = scratch("copy");
        let src = d.join("a.txt");
        std::fs::write(&src, "hello").unwrap();
        let out = d.join("out");
        std::fs::create_dir(&out).unwrap();
        let mut s = Session::new(doc4me_doc::Document::new());
        let r = run(&mut s, "file.copyFile", json!({"fileName": src.to_string_lossy(), "directory": out.to_string_lossy()})).unwrap();
        assert_eq!(std::fs::read_to_string(out.join("a.txt")).unwrap(), "hello");
        assert!(r["path"].as_str().unwrap().ends_with("a.txt"));
        // Already there: kept unless overwrite.
        std::fs::write(&src, "changed").unwrap();
        assert!(run(&mut s, "file.copyFile", json!({"fileName": src.to_string_lossy(), "directory": out.to_string_lossy()})).is_err());
        assert_eq!(std::fs::read_to_string(out.join("a.txt")).unwrap(), "hello");
        run(&mut s, "file.copyFile", json!({"fileName": src.to_string_lossy(), "directory": out.to_string_lossy(), "overwrite": true})).unwrap();
        assert_eq!(std::fs::read_to_string(out.join("a.txt")).unwrap(), "changed");
        // Onto itself, from a folder, into a file: refused.
        assert!(run(&mut s, "file.copyFile", json!({"fileName": src.to_string_lossy(), "directory": d.to_string_lossy(), "overwrite": true})).is_err());
        assert_eq!(std::fs::read_to_string(&src).unwrap(), "changed");
        assert!(run(&mut s, "file.copyFile", json!({"fileName": out.to_string_lossy(), "directory": d.to_string_lossy()})).is_err());
        assert!(run(&mut s, "file.copyFile", json!({"fileName": src.to_string_lossy(), "directory": src.to_string_lossy()})).is_err());
        // Outside the automation folders: refused before anything is touched.
        let roots = crate::file_access::AutomationRoots::new(Some(&out), Some(&out)).unwrap();
        let r = crate::file_access::confine(roots.as_ref(), || {
            run(&mut s, "file.copyFile", json!({"fileName": src.to_string_lossy(), "directory": out.to_string_lossy(), "overwrite": true}))
        });
        assert!(r.is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn find_files_by_name_text_and_subfolders() {
        let d = scratch("find");
        std::fs::write(d.join("alpha.txt"), "The quick brown fox").unwrap();
        std::fs::write(d.join("beta.txt"), "Nothing here").unwrap();
        std::fs::write(d.join("image.png"), [0u8; 4]).unwrap();
        std::fs::create_dir(d.join("sub")).unwrap();
        std::fs::write(d.join("sub").join("gamma.md"), "a QUICK note").unwrap();
        let docx = doc4me_docx::write(&doc4me_doc::Document::from_text("quick as a docx")).unwrap();
        std::fs::write(d.join("delta.docx"), docx).unwrap();
        let mut s = Session::new(doc4me_doc::Document::new());
        let names = |r: &Value| -> Vec<String> {
            r["files"].as_array().unwrap().iter().map(|f| Path::new(f["path"].as_str().unwrap()).file_name().unwrap().to_string_lossy().into_owned()).collect()
        };
        let path = d.to_string_lossy().to_string();
        // Every document (not the picture), this folder only.
        let r = run(&mut s, "file.findFiles", json!({"path": path})).unwrap();
        assert_eq!(names(&r), ["alpha.txt", "beta.txt", "delta.docx"]);
        assert_eq!(r["complete"], json!(true));
        assert!(r["files"][0]["size"].as_u64() == Some(19) && r["files"][0]["modified"].is_u64());
        // By name, then by the text they hold (a .docx is read too), with subfolders.
        let r = run(&mut s, "file.findFiles", json!({"path": path, "name": "*.PNG"})).unwrap();
        assert_eq!(names(&r), ["image.png"]);
        let r = run(&mut s, "file.findFiles", json!({"path": path, "text": "quick", "subfolders": true})).unwrap();
        assert_eq!(names(&r), ["alpha.txt", "delta.docx", "gamma.md"]);
        assert!(run(&mut s, "file.findFiles", json!({"path": d.join("alpha.txt").to_string_lossy()})).is_err(), "not a folder");
        // Confined elsewhere: refused.
        let sub = d.join("sub");
        let roots = crate::file_access::AutomationRoots::new(Some(&sub), None).unwrap();
        let r = crate::file_access::confine(roots.as_ref(), || run(&mut s, "file.findFiles", json!({"path": path})));
        assert!(r.is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
}
