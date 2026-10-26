//! Insert tab: pages, tables, pictures, shapes, links, bookmarks, headers/footers, text, symbols.

use doc4me_doc::para::{Float, InlineObject, ShapeKind, Wrap};
use doc4me_doc::props::{Align, CharProps, Rgb, TabAlign, TabStop, TextColor};
use doc4me_doc::{Block, Paragraph, PartKind, Path, Pos, StoryRef, Table, para_block};
use serde_json::{Value, json};

use super::{delete_selection, sel_result, split_para, type_text};
use crate::{CmdError, CmdResult, CommandSpec, Selection, Session, p};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("insert.pageBreak", "Page Break", "Insert › Pages", super::text::page_break),
        CommandSpec::new("insert.blankPage", "Blank Page", "Insert › Pages", |s, _| {
            type_text(s, "\u{000C}")?;
            let at = s.sel.focus.clone();
            let at = split_para(s, &at)?;
            s.doc.insert_text(&at, "\u{000C}", &CharProps::default())?;
            s.sel = Selection::caret(at);
            sel_result(s)
        }),
        CommandSpec::new("insert.coverPage", "Cover Page", "Insert › Pages", cover_page).params(r#"{"title"?: string, "subtitle"?: string, "author"?: string}"#),
        CommandSpec::new("insert.table", "Table", "Insert › Tables", table).params(r#"{"rows": n, "cols": n, "style"?: string}"#),
        CommandSpec::new("insert.picture", "Pictures", "Insert › Illustrations", picture).params(r#"{"path"?: string, "data"?: base64, "width"?: pt, "alt"?: string}"#),
        // Screen Clipping: the front end starts the system's screen clipping and inserts the clip
        // with `insert.picture`, so starting it changes nothing (no undo step, no unsaved change).
        CommandSpec::new("insert.screenshot", "Screenshot", "Insert › Illustrations", |s, _| {
            s.ui_requests.push(json!({"open": "screenClipping"}));
            sel_result(s)
        })
        .pure(),
        CommandSpec::new("insert.onlinePicture", "Online Pictures", "Insert › Illustrations", online_picture).params(
            r#"{"data"?: base64 (the picture; omit to show Online Pictures), "title"?: string, "creator"?: string, "license"?: "by|by-sa|by-nd|by-nc|by-nc-sa|by-nc-nd|cc0|pdm", "licenseVersion"?: string, "licenseUrl"?: url, "source"?: url (the picture's page), "attribution"?: bool (default true: a credit line under the picture), "width"?: pt}"#,
        ),
        CommandSpec::new("insert.model3d", "3D Models", "Insert › Illustrations", model3d).params(
            r#"{"path"?: string, "data"?: base64, "name"?: string (the file's name, for its format), "width"?: pt, "yaw"?: degrees, "pitch"?: degrees; .glb, .stl or .obj}"#,
        ),
        CommandSpec::new("insert.video", "Online Video", "Insert › Media", video)
            .params(r#"{"url"?: string (a video's web address or <iframe> embed code; omit to show Insert Video), "poster"?: base64 PNG/JPEG, "title"?: string}"#),
        CommandSpec::new("insert.icon", "Icons", "Insert › Illustrations", icon)
            .params(r#"{"id"?: string (omit to list the library), "color"?: "RRGGBB", "size"?: pt, "query"?: string, "category"?: string}"#),
        CommandSpec::new("insert.shape", "Shapes", "Insert › Illustrations", shape)
            .params(r#"{"kind": "rectangle|roundedRectangle|ellipse|triangle|diamond|line|arrow|star|heart|straightConnector|elbowConnector|curvedConnector", "width"?: pt, "height"?: pt, "fill"?: "RRGGBB", "stroke"?: "RRGGBB", "arrow"?: "none"|"start"|"end"|"both" (lines and connectors; glue a connector's ends with shape.connect)}"#),
        CommandSpec::new("insert.textBox", "Text Box", "Insert › Text", text_box)
            .params(r#"{"text"?: string, "width"?: pt, "height"?: pt}"#)
            .when(|s| {
                // As in Word: no text box inside a text box.
                matches!(s.sel.focus.story, StoryRef::Part(id) if s.doc.parts.get(&id).is_some_and(|p| p.kind == PartKind::TextBox))
                    .then_some("text boxes can't go inside a text box")
            }),
        CommandSpec::new("insert.link", "Link", "Insert › Links", link).key("Mod+K").params(r#"{"url": string, "text"?: string}"#),
        CommandSpec::new("insert.removeLink", "Remove Hyperlink", "Insert › Links", |s, _| super::format::apply(s, &|c| {
            c.link = None;
            if c.style.as_deref() == Some("Hyperlink") {
                c.style = None;
            }
        })),
        CommandSpec::new("insert.bookmark", "Bookmark", "Insert › Links", bookmark).params(r#"{"name": string}"#).key("Mod+Shift+F5"),
        CommandSpec::new("insert.header", "Header", "Insert › Header & Footer", |s, v| header_footer(s, v, true)).params(r#"{"text"?: string, "preset"?: "blank|blankThree|title"}"#),
        CommandSpec::new("insert.footer", "Footer", "Insert › Header & Footer", |s, v| header_footer(s, v, false)).params(r#"{"text"?: string, "preset"?: "blank|blankThree|pageNumber"}"#),
        CommandSpec::new("insert.pageNumber", "Page Number", "Insert › Header & Footer", page_number).params(r#"{"position"?: "top|bottom|current", "align"?: "left|center|right", "format"?: "x of y"}"#),
        CommandSpec::new("insert.editHeader", "Edit Header", "Insert › Header & Footer › Header", |s, _| edit_hf(s, true)).pure(),
        CommandSpec::new("insert.editFooter", "Edit Footer", "Insert › Header & Footer › Footer", |s, _| edit_hf(s, false)).pure(),
        CommandSpec::new("insert.closeHeader", "Close Header and Footer", "Header & Footer", |s, _| {
            s.sel = Selection::caret(s.doc.start_of(StoryRef::Body));
            sel_result(s)
        })
        .pure(),
        CommandSpec::new("insert.removeHeader", "Remove Header", "Insert › Header & Footer › Header", |s, _| {
            s.doc.last_section.headers = Default::default();
            for path in s.doc.para_paths(StoryRef::Body) {
                if s.doc.para(StoryRef::Body, &path).is_some_and(|p| p.section.is_some())
                    && let Ok(p) = s.doc.para_mut(StoryRef::Body, &path)
                    && let Some(sec) = p.section.as_mut()
                {
                    sec.headers = Default::default();
                }
            }
            sel_result(s)
        }),
        CommandSpec::new("insert.removeFooter", "Remove Footer", "Insert › Header & Footer › Footer", |s, _| {
            s.doc.last_section.footers = Default::default();
            sel_result(s)
        }),
        CommandSpec::new("insert.dateTime", "Date & Time", "Insert › Text", date_time).params(r#"{"format"?: "M/d/yyyy", "update"?: bool}"#),
        CommandSpec::new("insert.symbol", "Symbol", "Insert › Symbols", symbol).params(r#"{"char": string, "font"?: family (the symbol's own font)}"#),
        CommandSpec::new("insert.field", "Field", "Insert › Text › Quick Parts", field).key("Mod+F9").params(r#"{"instr": string, "result"?: string}"#),
        // Word's Ctrl+Alt+L: a LISTNUM field (numbering within a paragraph).
        // Word's Insert Number: a number written out in a numbering style, as an = field with a \*
        // format switch (which Word keeps when the field updates).
        CommandSpec::new("insert.number", "Number", "Insert › Symbols", insert_number).params(
            r#"{"number": 0-999999, "format"?: "Arabic|alphabetic|ALPHABETIC|roman|ROMAN|CardText|OrdText|Ordinal|ThaiArabic|ThaiCardText|ThaiLetter|ChineseNum1|ChineseNum2|ChineseNum3" (default Arabic; letters and Roman numerals from 1)}"#,
        ),
        CommandSpec::new("insert.listNumField", "Insert LISTNUM Field", "Insert › Text › Quick Parts", |s, _| {
            field(s, &json!({"instr": "LISTNUM", "result": "1)"}))
        })
        .key(super::non_mac("Mod+Alt+L")),
        // Word's Alt+Shift+D / T / P (Control+Shift+… in Word for Mac): DATE, TIME and PAGE fields.
        CommandSpec::new("insert.dateField", "Insert Date Field", "Insert › Text › Quick Parts", |s, _| field(s, &json!({"instr": "DATE"})))
            .key(super::mac_or("Ctrl+Shift+D", "Alt+Shift+D")),
        CommandSpec::new("insert.timeField", "Insert Time Field", "Insert › Text › Quick Parts", |s, _| field(s, &json!({"instr": "TIME"})))
            .key(super::mac_or("Ctrl+Shift+T", "Alt+Shift+T")),
        CommandSpec::new("insert.pageField", "Insert Page Field", "Insert › Text › Quick Parts", |s, _| field(s, &json!({"instr": "PAGE"})))
            .key(super::mac_or("Ctrl+Shift+P", "Alt+Shift+P")),
        CommandSpec::new("insert.dropCap", "Drop Cap", "Insert › Text", |s, v| {
            let lines = p::u64(v, "lines").unwrap_or(3).min(10) as u8;
            super::para::fmt(s, &|p| p.drop_cap = Some(lines))
        })
        .params(r#"{"lines"?: n (0 = none)}"#),
        CommandSpec::new("insert.horizontalLine", "Horizontal Line", "Home › Paragraph › Borders", |s, _| {
            super::para::fmt(s, &|p| {
                p.borders = Some(doc4me_doc::props::Borders { bottom: Some(doc4me_doc::props::Border::single(1.5)), ..Default::default() })
            })
        }),
        CommandSpec::new("insert.textFromFile", "Text from File", "Insert › Text › Object", |s, v| {
            let path = p::req_str(v, "path")?;
            let doc = crate::io::open_path(std::path::Path::new(path)).map_err(CmdError::Failed)?;
            let frag = doc.body_fragment();
            let at = delete_selection(s)?;
            let end = s.doc.insert_fragment(&at, &frag)?;
            s.sel = Selection::caret(end);
            sel_result(s)
        })
        .params(r#"{"path": string}"#),
    ]
}

/// Insert a block after the caret's paragraph (splitting it if the caret is mid-paragraph).
fn insert_block_at_caret(s: &mut Session, block: Block) -> Result<Path, CmdError> {
    let at = delete_selection(s)?;
    let len = s.doc.para_at(&at).map(|p| p.len()).unwrap_or(0);
    let insert_at = if at.off == 0 {
        at.path.clone()
    } else {
        let new = s.doc.split_paragraph(&at)?;
        if at.off >= len { at.path.with_last(at.path.last() + 1) } else { new.path }
    };
    s.doc.insert_block(at.story, &insert_at, block)?;
    // Word keeps a paragraph after a table.
    let next = insert_at.with_last(insert_at.last() + 1);
    if s.doc.block(at.story, &next).is_none() {
        s.doc.insert_block(at.story, &next, Block::Para(Paragraph::new()))?;
    }
    // An empty paragraph we inserted before? Remove if the table took its place at the start.
    Ok(insert_at)
}

fn table(s: &mut Session, v: &Value) -> CmdResult {
    let rows = p::u64(v, "rows").unwrap_or(2).clamp(1, 1000) as usize;
    let cols = p::u64(v, "cols").unwrap_or(2).clamp(1, 63) as usize;
    let width = s.doc.sections().first().map(|(_, sp)| sp.text_width()).unwrap_or(468.0);
    let mut t = Table::new(rows, cols, width);
    if let Some(st) = p::str(v, "style") {
        let id = s.doc.styles.find(st).map(|x| x.id.clone()).ok_or_else(|| CmdError::Params(format!("no table style `{st}`")))?;
        t.props.style = Some(id);
    }
    let story = s.sel.focus.story;
    let path = insert_block_at_caret(s, Block::Table(t))?;
    let mut first = path.0.clone();
    first.extend([0, 0, 0]);
    s.sel = Selection::caret(Pos { story, path: Path(first), off: 0 });
    super::references::auto_caption(s, "table")?;
    // The table's path after a caption above it.
    let at = s.sel.focus.path.cell().map_or(path.0, |(tp, _, _)| tp.0);
    Ok(json!({"table": at}))
}

/// Minimal base64 (standard alphabet, padding optional, whitespace ignored).
pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let mut buf = 0u32;
    let mut n = 0;
    for c in s.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            b'=' | b' ' | b'\n' | b'\r' | b'\t' => continue,
            _ => return None,
        } as u32;
        buf = (buf << 6) | v;
        n += 6;
        if n >= 8 {
            n -= 8;
            out.push((buf >> n) as u8);
            buf &= (1 << n) - 1;
        }
    }
    Some(out)
}

pub fn base64_encode(b: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(b.len().div_ceil(3) * 4);
    for ch in b.chunks(3) {
        let n =
            (ch.first().copied().unwrap_or(0) as u32) << 16 | (ch.get(1).copied().unwrap_or(0) as u32) << 8 | ch.get(2).copied().unwrap_or(0) as u32;
        for i in 0..4 {
            if i <= ch.len() {
                out.push(T[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn picture(s: &mut Session, v: &Value) -> CmdResult {
    let bytes = if let Some(d) = p::str(v, "data") {
        base64_decode(d).ok_or_else(|| CmdError::Params("bad base64 data".into()))?
    } else if let Some(path) = p::str(v, "path") {
        crate::io::read_file(path).map_err(CmdError::Failed)?
    } else {
        s.ui_requests.push(json!({"open": "insertPicture"}));
        return sel_result(s);
    };
    let r = place_picture(s, bytes, p::f32(v, "width"), p::str(v, "alt").unwrap_or(""))?;
    // A picture added to a drawing canvas is part of it, not captioned on its own.
    if r.get("canvasMember").is_none() {
        super::references::auto_caption(s, "picture")?;
    }
    Ok(r)
}

/// Insert › Online Pictures: a Creative Commons picture the front end found (Openverse) and
/// downloaded, with a credit line under it ("“Title” by Creator is licensed under CC BY 2.0",
/// the title linked to the picture's page and the licence to its deed), as the licences ask.
/// WebP pictures become JPEG or PNG, which every Word reads.
fn online_picture(s: &mut Session, v: &Value) -> CmdResult {
    let Some(data) = p::str(v, "data") else {
        s.ui_requests.push(json!({"open": "onlinePictures"}));
        return sel_result(s);
    };
    let mut bytes = base64_decode(data).ok_or_else(|| CmdError::Params("bad base64 data".into()))?;
    if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
        bytes = webp_to_jpeg_or_png(&bytes)?;
    }
    let text = |k: &str| p::str(v, k).map(str::trim).filter(|t| !t.is_empty()).map(|t| t.chars().take(300).collect::<String>());
    let title = text("title");
    let creator = text("creator");
    let licence = text("license").map(|l| licence_name(&l, text("licenseVersion").as_deref()));
    let web = |k: &str| text(k).filter(|u| doc4me_doc::link::is_web_or_mail_link(u) && !u.starts_with("mailto:"));
    let alt = match (&title, &creator) {
        (Some(t), Some(c)) => format!("{t} by {c}"),
        (Some(t), None) => t.clone(),
        (None, Some(c)) => format!("Picture by {c}"),
        (None, None) => "Online picture".to_string(),
    };
    let mut r = place_picture(s, bytes, p::f32(v, "width"), &alt)?;
    // A picture added to a selected Drawing Canvas has no paragraph of its own to credit under.
    if p::bool(v, "attribution").unwrap_or(true)
        && r.get("canvasMember").is_none()
        && let Some(licence) = licence
    {
        let at = s.sel.focus.clone();
        let base = CharProps { size: Some(8.0), ..CharProps::default() };
        let linked = |url: Option<String>| CharProps { link: url, ..base.clone() };
        let mut para = Paragraph::new();
        let mut push = |text: &str, props: CharProps| {
            let off = para.len();
            let _ = para.insert_text(off, text, &props);
        };
        match &title {
            Some(t) => push(&format!("“{t}”"), linked(web("source"))),
            None => push("This picture", linked(web("source"))),
        }
        if let Some(c) = &creator {
            push(&format!(" by {c}"), base.clone());
        }
        push(" is licensed under ", base.clone());
        push(&licence, linked(web("licenseUrl")));
        push(".", base.clone());
        para.mark = base;
        let next = at.path.with_last(at.path.last() + 1);
        s.doc.insert_block(at.story, &next, Block::Para(para))?;
        r["attribution"] = json!(true);
    }
    Ok(r)
}

/// "CC BY-SA 4.0", "CC0 1.0", "Public Domain Mark 1.0" for an Openverse licence code.
fn licence_name(code: &str, version: Option<&str>) -> String {
    let code = code.to_ascii_lowercase();
    let name = match code.as_str() {
        "cc0" => "CC0".to_string(),
        "pdm" => "Public Domain Mark".to_string(),
        c if c.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '+') => format!("CC {}", c.to_ascii_uppercase()),
        _ => "a Creative Commons licence".to_string(),
    };
    match version.filter(|v| v.len() <= 8 && v.chars().all(|c| c.is_ascii_digit() || c == '.')) {
        Some(v) => format!("{name} {v}"),
        None => name,
    }
}

/// A WebP picture as JPEG (or PNG when it has transparency), so older Word versions show it.
fn webp_to_jpeg_or_png(bytes: &[u8]) -> Result<Vec<u8>, CmdError> {
    let img =
        image::load_from_memory_with_format(bytes, image::ImageFormat::WebP).map_err(|e| CmdError::Failed(format!("not a readable picture: {e}")))?;
    let mut out = Vec::new();
    let mut cur = std::io::Cursor::new(&mut out);
    if img.color().has_alpha() {
        img.write_to(&mut cur, image::ImageFormat::Png)
    } else {
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cur, 90).encode_image(&img.to_rgb8())
    }
    .map_err(|e| CmdError::Failed(format!("couldn't convert the picture: {e}")))?;
    Ok(out)
}

/// Insert › 3D Models: the model in a glTF binary, STL or OBJ file (`crate::model3d`) as a
/// picture, 3 inches wide unless `width` says otherwise. Without a file it asks the front end to
/// pick one.
fn model3d(s: &mut Session, v: &Value) -> CmdResult {
    let (bytes, name) = if let Some(d) = p::str(v, "data") {
        (base64_decode(d).ok_or_else(|| CmdError::Params("bad base64 data".into()))?, p::str(v, "name").unwrap_or("").to_string())
    } else if let Some(path) = p::str(v, "path") {
        (crate::io::read_file(path).map_err(CmdError::Failed)?, path.to_string())
    } else {
        s.ui_requests.push(json!({"open": "insertModel3d"}));
        return sel_result(s);
    };
    // The file name, whichever system's separators the path has.
    let file = name.rsplit(['/', '\\']).next().unwrap_or("");
    let (stem, ext) = match file.rsplit_once('.') {
        Some((stem, ext)) => (stem, Some(ext.to_ascii_lowercase())),
        None => (file, None),
    };
    let mesh = crate::model3d::load(&bytes, ext.as_deref()).map_err(CmdError::Failed)?;
    let yaw = p::f32(v, "yaw").filter(|a| a.is_finite()).unwrap_or(-30.0);
    let pitch = p::f32(v, "pitch").filter(|a| a.is_finite()).unwrap_or(20.0).clamp(-90.0, 90.0);
    let png = crate::model3d::render(&mesh, 800, yaw, pitch);
    if png.is_empty() {
        return Err(CmdError::Failed("couldn't draw the model".into()));
    }
    let alt: String = if stem.is_empty() { "3D model" } else { stem }.chars().take(200).collect();
    let width = p::f32(v, "width").filter(|w| w.is_finite()).unwrap_or(216.0);
    let mut r = place_picture(s, png, Some(width), &alt)?;
    r["triangles"] = json!(mesh.triangles.len());
    Ok(r)
}

/// Insert › Online Video: a picture standing for a video on the web (`doc4me_doc::video`), the
/// poster frame given or drawn here, as wide as the video's player (never wider than the text
/// column). Without a `url` it asks the front end for Insert Video.
fn video(s: &mut Session, v: &Value) -> CmdResult {
    let Some(input) = p::str(v, "url") else {
        s.ui_requests.push(json!({"open": "insertVideo"}));
        return sel_result(s);
    };
    let video = doc4me_doc::video::WebVideo::from_input(input).map_err(CmdError::Params)?;
    let poster = match p::str(v, "poster") {
        Some(d) => base64_decode(d).ok_or_else(|| CmdError::Params("bad base64 poster".into()))?,
        None => doc4me_render::video_poster(video.w, video.h),
    };
    let alt = match p::str(v, "title").map(str::trim).filter(|t| !t.is_empty()) {
        Some(t) => t.chars().take(1000).collect(),
        None => video.watch_url().unwrap_or_default(),
    };
    let width = video.w as f32 * 0.75;
    let mut r = place(s, poster, Some(width), &alt, Some(std::sync::Arc::new(video)))?;
    r["video"] = json!(true);
    Ok(r)
}

/// Insert an encoded image at the selection as an inline picture, `width` points wide (default:
/// its natural size at 96 ppi), never wider than the text column.
fn place_picture(s: &mut Session, bytes: Vec<u8>, width: Option<f32>, alt: &str) -> CmdResult {
    place(s, bytes, width, alt, None)
}

fn place(s: &mut Session, bytes: Vec<u8>, width: Option<f32>, alt: &str, video: Option<std::sync::Arc<doc4me_doc::video::WebVideo>>) -> CmdResult {
    if bytes.len() > 200 << 20 {
        return Err(CmdError::Failed("image is larger than 200 MB".into()));
    }
    let (pw, ph) = doc4me_render::image_size(&bytes).ok_or_else(|| CmdError::Failed("not a supported image (PNG, JPEG, GIF, WebP, BMP)".into()))?;
    let ext = match bytes.get(..4) {
        Some([0x89, b'P', b'N', b'G']) => "png",
        Some([0xFF, 0xD8, ..]) => "jpeg",
        Some([b'G', b'I', b'F', _]) => "gif",
        Some([b'R', b'I', b'F', b'F']) => "webp",
        Some([b'B', b'M', ..]) => "bmp",
        _ => "png",
    };
    let key = s.doc.add_media(bytes, ext);
    // Natural size at 96 ppi, fitted to the text width.
    let max_w = s.doc.sections().first().map(|(_, sp)| sp.text_width()).unwrap_or(468.0);
    let (mut w, mut h) = (pw as f32 * 0.75, ph as f32 * 0.75);
    if let Some(want) = width {
        let k = want.max(4.0) / w.max(1.0);
        w *= k;
        h *= k;
    }
    if w > max_w {
        h *= max_w / w;
        w = max_w;
    }
    let obj = InlineObject::Image { media: key.clone(), w, h, alt: alt.to_string(), float: Float::default(), crop: [0.0; 4], ole: None, video };
    // A selected Drawing Canvas takes it in.
    if let Some(i) = super::canvas::add_member(s, obj.clone())? {
        return Ok(json!({"media": key, "canvasMember": i}));
    }
    let props = s.typing_props();
    let at = delete_selection(s)?;
    let end = s.doc.insert_object(&at, obj, &props)?;
    s.sel = Selection { anchor: at, focus: end };
    Ok(json!({"media": key, "width": w, "height": h}))
}

/// Insert › Icons: a library icon (original artwork drawn in code, `doc4me_render::icon_lib`)
/// rendered to a PNG picture `size` points square (default 1 inch) in `color` (default black).
/// Without an `id` it lists the library (narrowed by `query` and `category`) and asks a front end
/// for its icon picker.
fn icon(s: &mut Session, v: &Value) -> CmdResult {
    use doc4me_render::icon_lib;
    let Some(id) = p::str(v, "id") else {
        let category = p::str(v, "category").and_then(icon_lib::Category::parse);
        let icons: Vec<Value> = icon_lib::search(p::str(v, "query").unwrap_or(""), category)
            .iter()
            .map(|i| json!({"id": i.id, "name": i.name, "category": i.category.name(), "keywords": i.keywords}))
            .collect();
        s.ui_requests.push(json!({"open": "icons"}));
        return Ok(json!({"icons": icons}));
    };
    let icon = icon_lib::find(id).ok_or_else(|| CmdError::Params(format!("no icon {id:?} (insert.icon without an id lists them)")))?;
    let ink = match p::str(v, "color") {
        Some(c) => Rgb::parse(c).ok_or_else(|| CmdError::Params(format!("bad color {c:?}: use RRGGBB")))?,
        None => Rgb::BLACK,
    };
    let size = p::f32(v, "size").unwrap_or(72.0).clamp(4.0, 1584.0);
    // About 300 pixels per inch, so it prints sharply.
    let px = (size * 300.0 / 72.0).round().clamp(64.0, 1024.0) as u32;
    let png = icon_lib::render(icon, px, ink).to_png();
    if png.is_empty() {
        return Err(CmdError::Failed("couldn't draw the icon".into()));
    }
    let mut r = place_picture(s, png, Some(size), icon.name)?;
    r["icon"] = json!(icon.id);
    Ok(r)
}

fn shape(s: &mut Session, v: &Value) -> CmdResult {
    let kind: ShapeKind =
        serde_json::from_value(v.get("kind").cloned().unwrap_or(json!("rectangle"))).map_err(|e| CmdError::Params(e.to_string()))?;
    let w = p::f32(v, "width").unwrap_or(108.0).clamp(4.0, 2000.0);
    let h =
        p::f32(v, "height").unwrap_or(if kind == ShapeKind::Line { 1.0 } else { 72.0 }).clamp(if kind.is_connector() { 0.0 } else { 1.0 }, 2000.0);
    let fill = match p::str(v, "fill") {
        Some(c) => Rgb::parse(c),
        None if kind.is_open() => None,
        None => Some(Rgb(0x15, 0x60, 0x82)),
    };
    // Arrowheads on lines and connectors.
    let (arrow_start, arrow_end) = match p::str(v, "arrow") {
        Some("end") => (false, true),
        Some("start") => (true, false),
        Some("both") => (true, true),
        Some("none") | None => (false, false),
        Some(x) => return Err(CmdError::Params(format!("`arrow`: none, start, end or both, not `{x}`"))),
    };
    let extra = doc4me_doc::connector::ShapeExtra { arrow_start, arrow_end, ..Default::default() };
    // Connectors float in front of the text, so they can be glued to floating shapes.
    let float = if kind.is_connector() {
        Float { wrap: Wrap::InFrontOfText, ..Default::default() }
    } else {
        Float { wrap: Wrap::Inline, ..Default::default() }
    };
    let stroke = p::str(v, "stroke").and_then(Rgb::parse).or(Some(Rgb(0x0E, 0x40, 0x5A)));
    let obj =
        InlineObject::Shape { kind, w, h, fill, stroke, stroke_width: 1.0, float, story: None, freeform: None, effects: Default::default(), extra };
    // A selected Drawing Canvas takes it in (and stays selected for the next one).
    if super::canvas::add_member(s, obj.clone())?.is_some() {
        return sel_result(s);
    }
    let props = s.typing_props();
    let at = delete_selection(s)?;
    let end = s.doc.insert_object(&at, obj, &props)?;
    s.sel = Selection { anchor: at, focus: end };
    sel_result(s)
}

fn text_box(s: &mut Session, v: &Value) -> CmdResult {
    let text = p::str(v, "text").unwrap_or("").to_string();
    let w = p::f32(v, "width").unwrap_or(144.0).clamp(18.0, 2000.0);
    let h = p::f32(v, "height").unwrap_or(72.0).clamp(18.0, 2000.0);
    let id = s.doc.add_part(PartKind::TextBox, vec![para_block(Paragraph::with_text(&text, CharProps::default()))]);
    let obj = InlineObject::Shape {
        kind: ShapeKind::TextBox,
        w,
        h,
        fill: Some(Rgb::WHITE),
        stroke: Some(Rgb::BLACK),
        stroke_width: 0.75,
        float: Float::default(),
        story: Some(id),
        freeform: None,
        effects: Default::default(),
        extra: Default::default(),
    };
    // A selected Drawing Canvas takes it in; otherwise it goes at the caret.
    if super::canvas::add_member(s, obj.clone())?.is_none() {
        let props = s.typing_props();
        let at = delete_selection(s)?;
        s.doc.insert_object(&at, obj, &props)?;
    }
    // Like Word, type straight into the new box.
    s.sel = Selection::caret(s.doc.end_of(StoryRef::Part(id)));
    Ok(json!({"story": id}))
}

fn link(s: &mut Session, v: &Value) -> CmdResult {
    let Some(url) = p::str(v, "url").map(str::to_string) else {
        s.ui_requests.push(json!({"open": "link"}));
        return sel_result(s);
    };
    if url.is_empty() || url.len() > 4096 {
        return Err(CmdError::Params("bad url".into()));
    }
    let style = Some("Hyperlink".to_string());
    if s.sel.is_collapsed() {
        let text = p::str(v, "text").unwrap_or(&url).to_string();
        let mut props = s.typing_props();
        props.link = Some(url.clone());
        props.style = style;
        let at = s.sel.focus.clone();
        let end = s.doc.insert_text(&at, &text, &props)?;
        s.sel = Selection::caret(end);
        s.pending = Some(CharProps::default());
    } else {
        let (a, b) = s.sel.ordered();
        s.doc.format_range(&a, &b, &|c| {
            c.link = Some(url.clone());
            c.style = style.clone();
        })?;
    }
    sel_result(s)
}

fn bookmark(s: &mut Session, v: &Value) -> CmdResult {
    let name = p::req_str(v, "name")?.trim().to_string();
    if name.is_empty() || name.contains(' ') || name.len() > 40 || name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return Err(CmdError::Params("bookmark names start with a letter, have no spaces and at most 40 characters".into()));
    }
    // Replace an existing bookmark of the same name.
    for (_, pos) in s.doc.bookmarks().into_iter().filter(|(n, _)| *n == name).rev() {
        let para = s.doc.para_mut(pos.story, &pos.path)?;
        para.delete(pos.off, pos.off + doc4me_doc::para::OBJ.len_utf8())?;
        let offs: Vec<usize> = para.object_offsets();
        if let Some(e) = offs.into_iter().find(|o| matches!(para.object_at(*o), Some(InlineObject::BookmarkEnd { name: n }) if *n == name)) {
            para.delete(e, e + doc4me_doc::para::OBJ.len_utf8())?;
        }
    }
    let (a, b) = s.sel.ordered();
    let props = CharProps::default();
    let end = s.doc.insert_object(&b, InlineObject::BookmarkEnd { name: name.clone() }, &props)?;
    s.doc.insert_object(&a, InlineObject::BookmarkStart { name: name.clone() }, &props)?;
    let _ = end;
    sel_result(s)
}

fn hf_part(s: &mut Session, header: bool) -> u32 {
    let block = s.sel.focus.path.0.first().copied().unwrap_or(0) as usize;
    let sect = s.doc.section_mut(block).clone();
    let existing = if header { sect.headers.default } else { sect.footers.default };
    if let Some(id) = existing.filter(|id| s.doc.parts.contains_key(id)) {
        return id;
    }
    let style = if header { "Header" } else { "Footer" };
    let id = s.doc.add_part(if header { PartKind::Header } else { PartKind::Footer }, vec![para_block(Paragraph::new().styled(style))]);
    let sect = s.doc.section_mut(block);
    if header {
        sect.headers.default = Some(id);
    } else {
        sect.footers.default = Some(id);
    }
    id
}

fn header_footer(s: &mut Session, v: &Value, header: bool) -> CmdResult {
    let id = hf_part(s, header);
    let style = if header { "Header" } else { "Footer" };
    if let Some(text) = p::str(v, "text") {
        let blocks = text.split('\n').map(|l| para_block(Paragraph::with_text(l, CharProps::default()).styled(style))).collect();
        s.doc.set_story(StoryRef::Part(id), blocks)?;
    } else if let Some(preset) = p::str(v, "preset") {
        let mut para = Paragraph::new().styled(style);
        match preset {
            "blankThree" => {
                para.insert_text(0, "[Type here]\t[Type here]\t[Type here]", &CharProps::default())?;
            }
            "title" => {
                para.insert_text(0, if s.doc.core.title.is_empty() { "[Document title]" } else { &s.doc.core.title }, &CharProps::default())?;
                para.props.align = Some(Align::Center);
            }
            "pageNumber" => {
                para.props.align = Some(Align::Center);
                para.insert_object(
                    0,
                    InlineObject::Field { instr: "PAGE".into(), result: "1".into(), locked: false, code: false },
                    &CharProps::default(),
                )?;
            }
            _ => {
                para.insert_text(0, "[Type here]", &CharProps::default())?;
            }
        }
        s.doc.set_story(StoryRef::Part(id), vec![para_block(para)])?;
    }
    s.sel = Selection::caret(s.doc.start_of(StoryRef::Part(id)));
    Ok(json!({"story": id}))
}

fn edit_hf(s: &mut Session, header: bool) -> CmdResult {
    let id = hf_part(s, header);
    s.touch();
    s.sel = Selection::caret(s.doc.end_of(StoryRef::Part(id)));
    Ok(json!({"story": id}))
}

fn page_number(s: &mut Session, v: &Value) -> CmdResult {
    let position = p::str(v, "position").unwrap_or("bottom");
    let align = match p::str(v, "align").unwrap_or("center") {
        "left" => Align::Left,
        "right" => Align::Right,
        _ => Align::Center,
    };
    let x_of_y = p::str(v, "format").is_some_and(|f| f.contains("of"));
    let field = InlineObject::Field { instr: "PAGE".into(), result: "1".into(), locked: false, code: false };
    if position == "current" {
        let props = s.typing_props();
        let at = delete_selection(s)?;
        let end = s.doc.insert_object(&at, field, &props)?;
        s.sel = Selection::caret(end);
        return sel_result(s);
    }
    let header = position == "top";
    let id = hf_part(s, header);
    let mut para = Paragraph::new().styled(if header { "Header" } else { "Footer" });
    para.props.align = Some(align);
    para.props.tabs = Some(vec![TabStop { pos: 0.0, align: TabAlign::Clear, leader: Default::default() }]);
    if x_of_y {
        para.insert_text(0, "Page  of ", &CharProps::default())?;
        para.insert_object(5, field, &CharProps { bold: Some(true), ..Default::default() })?;
        let end = para.len();
        para.insert_object(
            end,
            InlineObject::Field { instr: "NUMPAGES".into(), result: "1".into(), locked: false, code: false },
            &CharProps { bold: Some(true), ..Default::default() },
        )?;
    } else {
        para.insert_object(0, field, &CharProps::default())?;
    }
    s.doc.set_story(StoryRef::Part(id), vec![para_block(para)])?;
    Ok(json!({"story": id}))
}

fn date_time(s: &mut Session, v: &Value) -> CmdResult {
    let fmt = p::str(v, "format").unwrap_or("M/d/yyyy");
    let text = format_date(fmt);
    if p::bool(v, "update").unwrap_or(false) {
        let props = s.typing_props();
        let at = delete_selection(s)?;
        let end = s.doc.insert_object(
            &at,
            InlineObject::Field { instr: format!("DATE \\@ \"{fmt}\""), result: text, locked: false, code: false },
            &props,
        )?;
        s.sel = Selection::caret(end);
    } else {
        type_text(s, &text)?;
    }
    sel_result(s)
}

/// Format today's date with a Word picture (`M/d/yyyy`, `MMMM d, yyyy`, `yyyy-MM-dd`, `dddd`…).
pub fn format_date(fmt: &str) -> String {
    // The computer's local date and time, not UTC.
    let now = super::now_local();
    let iso = super::iso_from_unix_secs(now);
    let y: i64 = iso.get(0..4).and_then(|x| x.parse().ok()).unwrap_or(2026);
    let m: usize = iso.get(5..7).and_then(|x| x.parse().ok()).unwrap_or(1);
    let d: u32 = iso.get(8..10).and_then(|x| x.parse().ok()).unwrap_or(1);
    let hh: u32 = iso.get(11..13).and_then(|x| x.parse().ok()).unwrap_or(0);
    let mm: u32 = iso.get(14..16).and_then(|x| x.parse().ok()).unwrap_or(0);
    let ss: u32 = iso.get(17..19).and_then(|x| x.parse().ok()).unwrap_or(0);
    format_date_parts(fmt, y, m, d, (hh, mm, ss), (now / 86_400) as i64)
}

/// Format a date and time with a Word picture; `days_since` (days since 1970-01-01) gives the weekday.
pub fn format_date_parts(fmt: &str, y: i64, m: usize, d: u32, (hh, mm, ss): (u32, u32, u32), days_since: i64) -> String {
    const MONTHS: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    const DAYS: [&str; 7] = ["Thursday", "Friday", "Saturday", "Sunday", "Monday", "Tuesday", "Wednesday"];
    let month = MONTHS.get(m.saturating_sub(1)).copied().unwrap_or("January");
    let day = DAYS.get(days_since.rem_euclid(7) as usize).copied().unwrap_or("Monday");
    let mut out = String::new();
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars.get(i).copied().unwrap_or(' ');
        // `am/pm` (any case) is one token: AM or PM.
        if chars.get(i..i + 5).is_some_and(|w| w.iter().collect::<String>().eq_ignore_ascii_case("am/pm")) {
            out.push_str(if hh < 12 { "AM" } else { "PM" });
            i += 5;
            continue;
        }
        let mut n = 1;
        while chars.get(i + n) == Some(&c) {
            n += 1;
        }
        match (c, n) {
            ('y', 4..) => out.push_str(&format!("{y:04}")),
            ('y', _) => out.push_str(&format!("{:02}", y % 100)),
            ('M', 4..) => out.push_str(month),
            ('M', 3) => out.push_str(month.get(..3).unwrap_or(month)),
            ('M', 2) => out.push_str(&format!("{m:02}")),
            ('M', 1) => out.push_str(&m.to_string()),
            ('d', 4..) => out.push_str(day),
            ('d', 3) => out.push_str(day.get(..3).unwrap_or(day)),
            ('d', 2) => out.push_str(&format!("{d:02}")),
            ('d', 1) => out.push_str(&d.to_string()),
            ('H', 2..) => out.push_str(&format!("{hh:02}")),
            ('H', _) => out.push_str(&hh.to_string()),
            ('h', _) => out.push_str(&(if hh.is_multiple_of(12) { 12 } else { hh % 12 }).to_string()),
            ('m', 2..) => out.push_str(&format!("{mm:02}")),
            ('m', _) => out.push_str(&mm.to_string()),
            ('s', 2..) => out.push_str(&format!("{ss:02}")),
            ('s', _) => out.push_str(&ss.to_string()),
            ('a' | 'A', _) => out.push_str(if hh < 12 { "AM" } else { "PM" }),
            _ => {
                for _ in 0..n {
                    out.push(c);
                }
            }
        }
        i += n;
    }
    out
}

/// Insert › Symbol: the character at the caret, in its own font when `font` names one (the rest of
/// the text keeps the caret's font).
fn symbol(s: &mut Session, v: &Value) -> CmdResult {
    let c = p::req_str(v, "char")?;
    if c.is_empty() || c.chars().count() > 64 {
        return Err(CmdError::Params("char: 1–64 characters".into()));
    }
    let font = p::str(v, "font").map(str::trim).filter(|f| !f.is_empty());
    let Some(font) = font else {
        type_text(s, c)?;
        return sel_result(s);
    };
    if font.len() > 256 {
        return Err(CmdError::Params("font: a family name".into()));
    }
    let base = s.typing_props();
    let mut props = base.clone();
    props.font = Some(font.to_string());
    s.pending = Some(props);
    let r = type_text(s, c);
    // Typing on after the symbol goes on in the text's own font.
    s.pending = Some(base);
    r?;
    sel_result(s)
}

/// Insert Number's number types: the `\*` switch and how it writes a number.
pub const NUMBER_FORMATS: [(&str, doc4me_doc::section::NumFormat); 14] = {
    use doc4me_doc::section::NumFormat as F;
    [
        ("Arabic", F::Decimal),
        ("alphabetic", F::LowerLetter),
        ("ALPHABETIC", F::UpperLetter),
        ("roman", F::LowerRoman),
        ("ROMAN", F::UpperRoman),
        ("CardText", F::CardinalText),
        ("OrdText", F::OrdinalText),
        ("Ordinal", F::Ordinal),
        ("ThaiArabic", F::ThaiNumbers),
        ("ThaiCardText", F::ThaiCounting),
        ("ThaiLetter", F::ThaiLetters),
        ("ChineseNum1", F::ChineseCountingThousand),
        ("ChineseNum2", F::ChineseLegalSimplified),
        ("ChineseNum3", F::IdeographDigital),
    ]
};

/// `insert.number`: `= 42 \* roman` showing "xlii".
fn insert_number(s: &mut Session, v: &Value) -> CmdResult {
    let n = p::u64(v, "number").filter(|n| *n <= 999_999).ok_or_else(|| CmdError::Params("`number` is 0 to 999999".into()))? as u32;
    let name = p::str(v, "format").unwrap_or("Arabic");
    let (switch, fmt) =
        NUMBER_FORMATS.iter().find(|(k, _)| *k == name).copied().ok_or_else(|| CmdError::Params(format!("no number format `{name}`")))?;
    if n == 0 && matches!(switch, "alphabetic" | "ALPHABETIC" | "roman" | "ROMAN" | "ThaiLetter") {
        return Err(CmdError::Params("letters and Roman numerals start at 1".into()));
    }
    let instr = if switch == "Arabic" { format!("= {n}") } else { format!("= {n} \\* {switch}") };
    // A field's CardText and OrdText are lower case ("twenty-one"), unlike list numbers.
    let result = if matches!(switch, "CardText" | "OrdText") { fmt.format(n).to_lowercase() } else { fmt.format(n) };
    field(s, &json!({"instr": instr, "result": result}))
}

fn field(s: &mut Session, v: &Value) -> CmdResult {
    let instr = p::req_str(v, "instr")?.trim().to_string();
    if instr.is_empty() || instr.len() > 4096 {
        return Err(CmdError::Params("bad field code".into()));
    }
    let result = p::str(v, "result").unwrap_or("").to_string();
    let props = s.typing_props();
    let at = delete_selection(s)?;
    let end = s.doc.insert_object(&at, InlineObject::Field { instr, result, locked: false, code: false }, &props)?;
    s.sel = Selection::caret(end);
    let _ = super::references::update_fields(s);
    sel_result(s)
}

fn cover_page(s: &mut Session, v: &Value) -> CmdResult {
    let title = p::str(v, "title").unwrap_or("Document Title").to_string();
    let subtitle = p::str(v, "subtitle").unwrap_or("Document subtitle").to_string();
    let author = p::str(v, "author").unwrap_or(&s.author).to_string();
    let accent = Rgb(0x15, 0x60, 0x82);
    let mut blocks = Vec::new();
    for _ in 0..8 {
        blocks.push(Block::Para(Paragraph::new()));
    }
    let mut t =
        Paragraph::with_text(&title, CharProps { size: Some(44.0), color: Some(TextColor::Rgb(accent)), ..Default::default() }).styled("Title");
    t.props.borders = Some(doc4me_doc::props::Borders {
        bottom: Some(doc4me_doc::props::Border { style: doc4me_doc::props::BorderStyle::Single, width: 2.0, color: Some(accent), space: 6.0 }),
        ..Default::default()
    });
    blocks.push(Block::Para(t));
    blocks.push(Block::Para(Paragraph::with_text(&subtitle, CharProps::default()).styled("Subtitle")));
    for _ in 0..14 {
        blocks.push(Block::Para(Paragraph::new()));
    }
    blocks
        .push(Block::Para(Paragraph::with_text(&author, CharProps { bold: Some(true), color: Some(TextColor::Rgb(accent)), ..Default::default() })));
    let mut date = Paragraph::with_text(&format_date("MMMM d, yyyy"), CharProps::default());
    date.insert_text(date.len(), "\u{000C}", &CharProps::default())?;
    blocks.push(Block::Para(date));
    let frag = doc4me_doc::edit::Fragment { blocks, ..Default::default() };
    let start = s.doc.start_of(StoryRef::Body);
    let at = s.doc.split_paragraph(&start)?;
    let _ = at;
    s.doc.insert_fragment(&start, &frag)?;
    s.sel = Selection::caret(s.doc.start_of(StoryRef::Body));
    sel_result(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_round_trip() {
        let data = b"hello world!?";
        assert_eq!(base64_decode(&base64_encode(data)).unwrap(), data);
        assert!(base64_decode("!!!").is_none());
    }

    /// Insert › Symbol with a font: only the symbol takes it, the caret's font carries on after.
    #[test]
    fn symbol_takes_its_own_font() {
        let mut s = Session::new(doc4me_doc::Document::from_text("ab"));
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("insert.symbol", &json!({"char": "\u{2665}", "font": "DejaVu Sans"})).unwrap();
        s.run("text.insert", &json!({"text": "c"})).unwrap();
        let p = s.doc.para(StoryRef::Body, &Path::top(0)).unwrap();
        assert_eq!(p.text, "ab\u{2665}c");
        assert_eq!(p.props_of_char(2).font.as_deref(), Some("DejaVu Sans"), "the symbol");
        assert_eq!(p.props_of_char(5).font, None, "typing after it (byte offsets)");
        assert!(s.run("insert.symbol", &json!({"char": ""})).is_err());
    }

    #[test]
    fn date_pictures() {
        let d = format_date("yyyy-MM-dd");
        assert_eq!(d.len(), 10);
        assert!(format_date("MMMM d, yyyy").contains(", "));
        let t = format_date("h:mm:ss am/pm");
        assert!(t.ends_with(" AM") || t.ends_with(" PM"), "{t}");
        assert_eq!(t.matches(':').count(), 2, "{t}");
        use super::super::references::date_picture;
        assert_eq!(date_picture(r#"DATE \@ "MMMM d, yyyy" \* MERGEFORMAT"#).as_deref(), Some("MMMM d, yyyy"));
        assert_eq!(date_picture(r#"TIME \@ HH:mm"#).as_deref(), Some("HH:mm"));
        assert_eq!(date_picture("DATE"), None);
    }
}
