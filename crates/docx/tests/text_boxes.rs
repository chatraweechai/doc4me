//! Text boxes' text direction, alignment and links (`wps:bodyPr/@vert`, `@anchor`;
//! `wps:txbx/@id`, `wps:linkedTxbx`). Fixtures are hand-written from ECMA-376 Part 1 §20.4
//! (DrawingML WordprocessingML drawing) and §21.1.2.1.1 (`bodyPr`).

use std::io::{Read, Write};

use doc4me_doc::para::ShapeKind;
use doc4me_doc::props::{TextVert, VAlign};
use doc4me_doc::{Document, InlineObject, PartKind, TextBody, para_block};

fn zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut zw = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, bytes) in entries {
        zw.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        zw.write_all(bytes).unwrap();
    }
    zw.finish().unwrap().into_inner()
}

fn document_xml(bytes: &[u8]) -> String {
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    let mut s = String::new();
    z.by_name("word/document.xml").unwrap().read_to_string(&mut s).unwrap();
    s
}

/// The text boxes' stories in body order.
fn boxes(d: &Document) -> Vec<u32> {
    let mut out = Vec::new();
    d.objects_in_reading_order(&mut |o| out.extend(o.text_box()));
    out.retain(|id| d.parts.get(id).is_some_and(|p| p.kind == PartKind::TextBox));
    out
}

fn text_box(d: &mut Document, text: &str, body: TextBody) -> InlineObject {
    let id = d.add_part(PartKind::TextBox, vec![para_block(doc4me_doc::Paragraph::with_text(text, Default::default()))]);
    d.parts.get_mut(&id).unwrap().body = body;
    InlineObject::Shape {
        kind: ShapeKind::TextBox,
        w: 144.0,
        h: 72.0,
        fill: None,
        stroke: None,
        stroke_width: 0.75,
        float: Default::default(),
        story: Some(id),
        effects: Default::default(),
        freeform: None,
        extra: Default::default(),
    }
}

fn tiny_png() -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([1, 2, 3, 255]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png).unwrap();
    out.into_inner()
}

#[test]
fn direction_alignment_and_links_round_trip() {
    let mut d = Document::from_text("a\nb\nc");
    let a = text_box(&mut d, "Flows on", TextBody { vert: TextVert::Vert, anchor: VAlign::Center, next: None, insets: None });
    // Margins of the box's own (none on the left and top, as forms often have).
    let b = text_box(&mut d, "", TextBody { insets: Some([0, 0, 91_440, 12_700]), ..Default::default() });
    let c = text_box(&mut d, "", TextBody { vert: TextVert::Stacked, anchor: VAlign::Bottom, next: None, insets: None });
    for (i, o) in [a, b, c].into_iter().enumerate() {
        d.insert_object(&doc4me_doc::Pos::body(i, 1), o, &Default::default()).unwrap();
    }
    let ids = boxes(&d);
    d.parts.get_mut(&ids[0]).unwrap().body.next = Some(ids[1]);
    d.parts.get_mut(&ids[1]).unwrap().body.next = Some(ids[2]);
    let bytes = doc4me_docx::write(&d).unwrap();
    let xml = document_xml(&bytes);
    assert!(xml.contains(r#"<wps:txbx id="1">"#), "{xml}");
    assert!(xml.contains(r#"<wps:linkedTxbx id="1" seq="1"/>"#) && xml.contains(r#"<wps:linkedTxbx id="1" seq="2"/>"#));
    assert!(xml.contains(r#"<wps:bodyPr vert="vert" anchor="ctr"/>"#) && xml.contains(r#"<wps:bodyPr vert="wordArtVert" anchor="b"/>"#));
    assert!(xml.contains(r#"<wps:bodyPr lIns="0" tIns="0" rIns="91440" bIns="12700"/>"#), "{xml}");
    let r = doc4me_docx::read(&bytes).unwrap();
    let got = boxes(&r);
    assert_eq!(r.text_box_chains(), vec![got.clone()]);
    assert_eq!(r.plain_text(doc4me_doc::StoryRef::Part(got[0])), "Flows on");
    let body = |id: u32| r.parts.get(&id).unwrap().body;
    assert_eq!((body(got[0]).vert, body(got[0]).anchor), (TextVert::Vert, VAlign::Center));
    assert_eq!((body(got[2]).vert, body(got[2]).anchor), (TextVert::Stacked, VAlign::Bottom));
    assert_eq!((body(got[1]).insets, body(got[0]).insets), (Some([0, 0, 91_440, 12_700]), None));
}

/// A paragraph with an inline text box whose `wps:wsp` children are `inner`.
fn wsp(inner: &str) -> String {
    format!(
        r#"<w:p><w:r><w:drawing><wp:inline><wp:extent cx="1828800" cy="914400"/><wp:docPr id="1" name="Box"/><a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr txBox="1"/><wps:spPr><a:prstGeom prst="rect"/></wps:spPr>{inner}</wps:wsp></a:graphicData></a:graphic></wp:inline></w:drawing></w:r></w:p>"#
    )
}

#[test]
fn hostile_links_read_without_chains_running_wild() {
    let head = wsp(
        r#"<wps:txbx id="7"><w:txbxContent><w:p><w:r><w:t>Text</w:t></w:r></w:p></w:txbxContent></wps:txbx><wps:bodyPr vert="eaVert" anchor="dist"/>"#,
    );
    // A repeated seq, a link to a chain with no first box, a seq that isn't a number.
    let body = [
        head,
        wsp(r#"<wps:linkedTxbx id="7" seq="1"/><wps:bodyPr/>"#),
        wsp(r#"<wps:linkedTxbx id="7" seq="1"/><wps:bodyPr/>"#),
        wsp(r#"<wps:linkedTxbx id="9" seq="1"/><wps:bodyPr/>"#),
        wsp(r#"<wps:linkedTxbx id="7" seq="-3"/><wps:bodyPr/>"#),
    ]
    .concat();
    let root = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let doc = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><w:body>{body}</w:body></w:document>"#
    );
    let r = doc4me_docx::read(&zip(&[("_rels/.rels", root.as_bytes()), ("word/document.xml", doc.as_bytes())])).unwrap();
    let got = boxes(&r);
    assert_eq!(got.len(), 4, "every box with a story is read (a bad seq links nothing)");
    assert_eq!(r.text_box_chains(), vec![vec![got[0], got[1]]], "one chain: the first box and the first seq 1");
    let b = r.parts.get(&got[0]).unwrap().body;
    assert_eq!((b.vert, b.anchor), (TextVert::Vert, VAlign::Center));
}

#[test]
fn image_inside_text_box_keeps_its_text_box_story() {
    let png = tiny_png();
    let rels = r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdImage" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/preview.png"/></Relationships>"#;
    let body = wsp(
        r#"<wps:txbx><w:txbxContent><w:p><w:r><w:t>Credit </w:t></w:r><w:r><w:drawing><wp:inline><wp:extent cx="9525" cy="9525"/><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic><pic:blipFill><a:blip r:embed="rIdImage"/></pic:blipFill></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r></w:p></w:txbxContent></wps:txbx>"#,
    );
    let root = r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let doc = format!(
        r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><w:body>{body}</w:body></w:document>"#
    );
    let types =
        r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="png" ContentType="image/png"/></Types>"#;
    let package = zip(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", root.as_bytes()),
        ("word/document.xml", doc.as_bytes()),
        ("word/_rels/document.xml.rels", rels.as_bytes()),
        ("word/media/preview.png", &png),
    ]);
    let read = |bytes: &[u8]| {
        let d = doc4me_docx::read(bytes).unwrap();
        let ids = boxes(&d);
        assert_eq!(ids.len(), 1);
        assert_eq!(d.plain_text(doc4me_doc::StoryRef::Part(ids[0])), "Credit ");
        let story = d.parts.get(&ids[0]).unwrap();
        assert!(story.blocks.iter().filter_map(|b| b.as_para()).any(|p| p.objects.iter().any(|o| matches!(o, InlineObject::Image { .. }))));
        d
    };
    let original = read(&package);
    let saved = doc4me_docx::write(&original).unwrap();
    read(&saved);
}
