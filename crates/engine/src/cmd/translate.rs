//! Review › Language › Translate: what the Translator pane sends to a translation service and
//! how the answers come back into the document. The engine never goes on the network itself:
//! the front end asks a free service (MyMemory, or a LibreTranslate server of the user's
//! choosing) and hands the translations back here.
//!
//! - `review.translate` opens the pane and gives the text to translate: the selection, or every
//!   body paragraph with text (`"scope": "document"`), with the language it is written in.
//! - The pane puts a translated selection in with `edit.pasteText` (the selection's formatting
//!   stays), and a translated document with `review.translateDocument`, which replaces the
//!   document with a translated copy as Word opens one: each paragraph keeps its style, its
//!   paragraph formatting, the formatting of its first character and its pictures.

use doc4me_doc::para::OBJ;
use doc4me_doc::{Path, StoryRef};
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Session, p};

/// Most paragraphs one document translation sends.
pub const MAX_PARAGRAPHS: usize = 20_000;
/// Longest text one request may carry, in bytes (MyMemory's limit is 500).
pub const MAX_PIECE: usize = 480;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("review.translate", "Translate", "Review › Language", translate)
            .params(r#"{"scope"?: "selection"|"document"} → {"text": the selection, "language": tag} or {"paragraphs": [{"path", "text", "language"}]}; from the ribbon it opens the Translator pane"#)
            .pure(),
        CommandSpec::new("review.translateDocument", "Translate Document", "Review › Language › Translate", translate_document)
            .params(r#"{"paragraphs": [{"path": path, "text": translated text}], "language"?: tag (the translation's language)} — replaces the document with a translated copy"#)
            .pure(),
    ]
}

fn translate(s: &mut Session, v: &Value) -> CmdResult {
    match p::str(v, "scope") {
        Some("document") => {
            let paragraphs: Vec<Value> = s
                .doc
                .para_paths(StoryRef::Body)
                .into_iter()
                .filter_map(|path| {
                    let para = s.doc.para(StoryRef::Body, &path)?;
                    let text = para.plain_text();
                    let text = text.trim();
                    text.chars().any(char::is_alphanumeric).then(|| json!({"path": path, "text": text, "language": language_of(s, &path)}))
                })
                .take(MAX_PARAGRAPHS)
                .collect();
            Ok(json!({"paragraphs": paragraphs}))
        }
        Some("selection") | None => {
            if v.get("scope").is_none() {
                s.ui_requests.push(json!({"open": "translator"}));
            }
            let text = s.selected_text();
            let language = language_of(s, &s.sel.focus.path.clone());
            Ok(json!({"text": text, "language": language}))
        }
        Some(other) => Err(CmdError::Params(format!("scope {other:?}: use \"selection\" or \"document\""))),
    }
}

/// The proofing language of the paragraph's first character (or the document's default), as a
/// tag like `en-US`; empty when none is set.
fn language_of(s: &Session, path: &Path) -> String {
    let story = s.sel.focus.story;
    let story = if s.doc.para(story, path).is_some() { story } else { StoryRef::Body };
    s.doc
        .para(story, path)
        .and_then(|para| para.runs.first().and_then(|r| r.props.lang.clone()).or_else(|| para.mark.lang.clone()))
        .or_else(|| s.doc.styles.default_chr.lang.clone())
        .unwrap_or_default()
}

fn translate_document(s: &mut Session, v: &Value) -> CmdResult {
    let items = v.get("paragraphs").and_then(Value::as_array).ok_or_else(|| CmdError::Params("`paragraphs` is a list of {path, text}".into()))?;
    let mut doc = s.doc.clone();
    let mut changed = 0;
    for item in items.iter().take(MAX_PARAGRAPHS) {
        let (Some(path), Some(text)) =
            (item.get("path").and_then(|p| serde_json::from_value::<Path>(p.clone()).ok()), item.get("text").and_then(Value::as_str))
        else {
            continue;
        };
        let Ok(para) = doc.para_mut(StoryRef::Body, &path) else { continue };
        // The first character's formatting for the new text (not a picture's).
        let props = para
            .run_ranges()
            .find(|(r, _)| para.text.get(r.clone()).is_some_and(|t| t.chars().any(|c| c != OBJ)))
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| para.mark.clone());
        let keep = para.object_offsets();
        let len = para.len();
        para.delete_except(0, len, &keep).map_err(|e| CmdError::Failed(e.to_string()))?;
        let at = para.len();
        let text: String = text.chars().filter(|c| *c != OBJ && *c != '\r' && *c != '\n').take(100_000).collect();
        para.insert_text(at, &text, &props).map_err(|e| CmdError::Failed(e.to_string()))?;
        if let Some(tag) = p::str(v, "language").filter(|t| t.len() <= 35 && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')) {
            let end = para.len();
            let _ = para.format(at, end, &|c| c.lang = Some(tag.to_string()));
        }
        para.touch();
        changed += 1;
    }
    s.set_document(doc);
    s.path = None;
    s.dirty = true;
    Ok(json!({"translated": changed}))
}

/// `text` cut into pieces of at most `max` bytes for a translation request, at sentence ends,
/// then spaces, then any character; each piece with what joins it to the next (the space it was
/// cut at, or nothing).
pub fn pieces(text: &str, max: usize) -> Vec<(String, &'static str)> {
    let max = max.max(8);
    let mut out = Vec::new();
    let mut rest = text.trim();
    while !rest.is_empty() {
        if rest.len() <= max {
            out.push((rest.to_string(), ""));
            break;
        }
        let mut cut = max;
        while !rest.is_char_boundary(cut) {
            cut -= 1;
        }
        let head = &rest[..cut];
        let sentence = head
            .rfind(['.', '!', '?', '。', '！', '？', '\u{0E2F}'])
            .map(|i| i + head[i..].chars().next().map_or(1, char::len_utf8))
            .filter(|&i| i > max / 3);
        let (end, join) = match (sentence, head.rfind(' ').filter(|&i| i > max / 3)) {
            (Some(i), _) => (i, " "),
            (None, Some(i)) => (i, " "),
            (None, None) => (cut, ""),
        };
        out.push((rest[..end].trim().to_string(), join));
        rest = rest[end..].trim_start();
    }
    out.retain(|(p, _)| !p.is_empty());
    if let Some(last) = out.last_mut() {
        last.1 = "";
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_text_is_cut_where_sentences_and_words_end() {
        let text = "First sentence here. Second one is a bit longer than that. Third.";
        let p = pieces(text, 30);
        assert!(p.iter().all(|(t, _)| t.len() <= 30), "{p:?}");
        assert_eq!(p.iter().map(|(t, j)| format!("{t}{j}")).collect::<String>(), text);
        // Thai has no spaces: cut on character boundaries, joined with nothing.
        let thai = "ภาษาไทยไม่มีการเว้นวรรคระหว่างคำ".repeat(10);
        let p = pieces(&thai, 50);
        assert!(p.iter().all(|(t, j)| t.len() <= 50 && j.is_empty()));
        assert_eq!(p.iter().map(|(t, _)| t.as_str()).collect::<String>(), thai);
        assert!(pieces("   ", 10).is_empty());
    }

    #[test]
    fn a_translated_document_keeps_its_structure() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Hello world.\n\nGood morning."));
        s.run("format.bold", &json!({})).ok();
        let png = crate::cmd::insert::base64_encode(&doc4me_render::video_poster(200, 100));
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("insert.picture", &json!({"data": png})).unwrap();
        let r = s.run("review.translate", &json!({"scope": "document"})).unwrap();
        let paras = r["paragraphs"].as_array().unwrap();
        assert_eq!(paras.len(), 2, "empty paragraphs aren't sent: {paras:?}");
        assert_eq!(paras[1]["text"], "Good morning.");
        let translations: Vec<Value> = paras.iter().zip(["สวัสดีชาวโลก", "อรุณสวัสดิ์"]).map(|(p, t)| json!({"path": p["path"], "text": t})).collect();
        let r = s.run("review.translateDocument", &json!({"paragraphs": translations, "language": "th-TH"})).unwrap();
        assert_eq!(r["translated"], 2);
        let text = s.doc.plain_text(StoryRef::Body);
        assert!(text.contains("สวัสดีชาวโลก") && text.contains("อรุณสวัสดิ์") && !text.contains("Hello"), "{text}");
        let last = s.doc.para_paths(StoryRef::Body).pop().unwrap();
        let para = s.doc.para(StoryRef::Body, &last).unwrap();
        assert_eq!(para.object_offsets().len(), 1, "the picture stays");
        assert_eq!(para.runs.last().and_then(|r| r.props.lang.clone()).as_deref(), Some("th-TH"));
        assert!(s.dirty && s.path.is_none(), "a new, unsaved document");
        // Junk is refused or skipped, never a panic.
        assert!(s.run("review.translateDocument", &json!({})).is_err());
        assert!(s.run("review.translateDocument", &json!({"paragraphs": [{"path": "x", "text": 1}, {"path": [99, 99], "text": "a"}]})).is_ok());
        assert!(s.run("review.translate", &json!({"scope": "everything"})).is_err());
    }
}
