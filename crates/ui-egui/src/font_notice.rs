//! The notice shown when an opened document uses a font Doc4Me has no metrics for and that isn't
//! installed (TH SarabunIT๙, `doc4me_engine::cmd::file::unmatched_fonts`): the bundled Sarabun
//! draws it at its own size, so lines and pages can break differently than with the real font.

use serde_json::Value;

use crate::WordApp;

/// A `fontsSubstituted` request from File › Open: say which fonts are drawn with what.
pub(crate) fn request(app: &mut WordApp, req: &Value) {
    let fonts: Vec<&str> = req.get("fontsSubstituted").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).collect();
    if fonts.is_empty() {
        return;
    }
    let with = req.get("with").and_then(Value::as_str).unwrap_or("Sarabun");
    let msg = crate::i18n::fmt(
        tl!(
            "{fonts} isn't installed on this computer, so this document is shown with {with}. Lines and pages may break differently than with the original font."
        ),
        &[("fonts", &fonts.join(", ")), ("with", with)],
    );
    app.status(msg.clone());
    app.font_notice = Some(msg);
}

/// The notice, until the user closes it.
pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    let Some(msg) = app.font_notice.as_deref() else { return };
    let mut open = true;
    let mut ok = false;
    egui::Window::new(tl!("Font not installed"))
        .id(egui::Id::new("font-notice"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 140.0))
        .show(ctx, |ui| {
            ui.set_max_width(380.0);
            ui.label(msg);
            ui.add_space(6.0);
            if ui.add(crate::widgets::primary_button(ui, tl!("OK"))).clicked() {
                ok = true;
            }
        });
    if ok || !open {
        app.font_notice = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn substituted_fonts_are_reported_until_closed() {
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), crate::Services::default());
        request(&mut app, &json!({"fontsSubstituted": [], "with": "Sarabun"}));
        assert!(app.font_notice.is_none());
        app.handle_request(&json!({"fontsSubstituted": ["TH SarabunIT๙"], "with": "Sarabun"}));
        let msg = app.font_notice.clone().unwrap();
        assert!(msg.contains("TH SarabunIT๙") && msg.contains("Sarabun"), "{msg}");
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| *m == msg));
    }
}
