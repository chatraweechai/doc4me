//! Comment times as Word 2019 and later write them: `w:date` is the author's local time (marked
//! `Z` all the same) and the UTC time is in commentsExtensible.xml, keyed by the durable id
//! commentsIds.xml gives the paragraph id of the comment's last paragraph. Fixtures are
//! hand-written.

use std::io::Write;

fn zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut zw = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, bytes) in entries {
        zw.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        zw.write_all(bytes).unwrap();
    }
    zw.finish().unwrap().into_inner()
}

const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const W14: &str = "http://schemas.microsoft.com/office/word/2010/wordml";

/// Two comments (ids 0 and 1, last paragraphs 0A000001 and 0A000002) and, when `utc`, the
/// commentsIds and commentsExtensible parts (`ids` and `cex` are their bodies).
fn docx(utc: bool, ids: &str, cex: &str) -> Vec<u8> {
    let root = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let mut rels = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" Target="comments.xml"/>"#,
    );
    if utc {
        rels.push_str(
            r#"<Relationship Id="rId2" Type="http://schemas.microsoft.com/office/2016/09/relationships/commentsIds" Target="commentsIds.xml"/>"#,
        );
        rels.push_str(r#"<Relationship Id="rId3" Type="http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible" Target="commentsExtensible.xml"/>"#);
    }
    rels.push_str("</Relationships>");
    let doc = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="{W}"><w:body><w:p><w:commentRangeStart w:id="0"/><w:commentRangeStart w:id="1"/><w:r><w:t>Text</w:t></w:r><w:commentRangeEnd w:id="0"/><w:commentRangeEnd w:id="1"/><w:r><w:commentReference w:id="0"/></w:r><w:r><w:commentReference w:id="1"/></w:r></w:p></w:body></w:document>"#
    );
    let comments = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><w:comments xmlns:w="{W}" xmlns:w14="{W14}"><w:comment w:id="0" w:author="Ann" w:date="2025-10-10T14:53:00Z"><w:p w14:paraId="0A000001"><w:r><w:t>First</w:t></w:r></w:p></w:comment><w:comment w:id="1" w:author="Ben" w:date="2025-10-11T09:00:00Z"><w:p w14:paraId="0A000002"><w:r><w:t>Second</w:t></w:r></w:p></w:comment></w:comments>"#
    );
    let ids = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><w16cid:commentsIds xmlns:w16cid="http://schemas.microsoft.com/office/word/2016/wordml/cid">{ids}</w16cid:commentsIds>"#
    );
    let cex = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><w16cex:commentsExtensible xmlns:w16cex="http://schemas.microsoft.com/office/word/2018/wordml/cex">{cex}</w16cex:commentsExtensible>"#
    );
    zip(&[
        ("_rels/.rels", root.as_bytes()),
        ("word/document.xml", doc.as_bytes()),
        ("word/_rels/document.xml.rels", rels.as_bytes()),
        ("word/comments.xml", comments.as_bytes()),
        ("word/commentsIds.xml", ids.as_bytes()),
        ("word/commentsExtensible.xml", cex.as_bytes()),
    ])
}

fn dates(d: &doc4me_doc::Document) -> Vec<(String, String, String)> {
    d.comments.values().map(|c| (c.author.clone(), c.date.clone(), c.date_utc.clone())).collect()
}

#[test]
fn comment_utc_times_come_from_comments_extensible() {
    let ids = r#"<w16cid:commentId w16cid:paraId="0A000001" w16cid:durableId="1B2C3D4E"/><w16cid:commentId w16cid:paraId="0A000002" w16cid:durableId="2C3D4E5F"/>"#;
    // Durable ids match whatever their case; a dateUtc for an unknown id is ignored.
    let cex = r#"<w16cex:commentExtensible w16cex:durableId="1b2c3d4e" w16cex:dateUtc="2025-10-10T07:53:00Z"/><w16cex:commentExtensible w16cex:durableId="7FFFFFFF" w16cex:dateUtc="2001-01-01T00:00:00Z"/>"#;
    let d = doc4me_docx::read(&docx(true, ids, cex)).unwrap();
    assert_eq!(
        dates(&d),
        [
            ("Ann".to_string(), "2025-10-10T14:53:00Z".to_string(), "2025-10-10T07:53:00Z".to_string()),
            ("Ben".to_string(), "2025-10-11T09:00:00Z".to_string(), String::new()),
        ]
    );
}

#[test]
fn without_the_extension_parts_only_the_word_date_is_known() {
    let d = doc4me_docx::read(&docx(false, "", "")).unwrap();
    assert!(d.comments.values().all(|c| c.date_utc.is_empty() && !c.date.is_empty()));
    // A hostile dateUtc (too long) is not kept.
    let ids = r#"<w16cid:commentId w16cid:paraId="0A000001" w16cid:durableId="1B2C3D4E"/>"#;
    let long = "9".repeat(100_000);
    let cex = format!(r#"<w16cex:commentExtensible w16cex:durableId="1B2C3D4E" w16cex:dateUtc="{long}"/>"#);
    let d = doc4me_docx::read(&docx(true, ids, &cex)).unwrap();
    assert!(d.comments.values().all(|c| c.date_utc.is_empty()));
}
