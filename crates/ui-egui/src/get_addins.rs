//! Insert › Add-ins › Get Add-ins: the add-in catalog (`addins/catalog.json` in Doc4Me's
//! repository, see [`doc4me_engine::cmd::addins`]) and the add-ins installed here.
//!
//! **Store** downloads the catalog and, on Add, the pack, which the engine checks against the
//! SHA-256 the catalog gives and against what add-ins may do before installing its macros.
//! **My Add-ins** runs or removes them. Installed packs are kept with the preferences and
//! installed again at start-up.

use doc4me_engine::cmd::addins::{self, Entry};
use serde_json::json;

use crate::WordApp;
use crate::net::{FetchRequest, Pending};
use crate::theme::Tokens;

/// The Get Add-ins window.
#[derive(Default)]
pub struct Store {
    pub mine: bool,
    catalog: Option<Pending>,
    pub entries: Vec<Entry>,
    /// The pack being downloaded: its catalog entry.
    adding: Option<(Entry, Pending)>,
    pub message: Option<String>,
}

/// Open the window (`addins.get` without parameters) and ask for the catalog.
pub(crate) fn open(app: &mut WordApp) {
    if app.addins_store.is_some() {
        return;
    }
    let mut store = Store::default();
    match crate::net::start(app, FetchRequest::get(addins::CATALOG_URL, addins::MAX_CATALOG_BYTES)) {
        Ok(rx) => store.catalog = Some(Pending::new(rx)),
        Err(e) => store.message = Some(crate::i18n::fmt(tl!("Couldn't get the add-in list: {reason}"), &[("reason", &e)])),
    }
    store.mine = store.catalog.is_none() && !app.ui.addins.is_empty();
    app.addins_store = Some(store);
}

/// The installed packs' ids and names, and their macros (installed name, shown name).
pub fn installed(app: &WordApp) -> Vec<(String, String, Vec<(String, String)>)> {
    let reg = app.session.registry.clone();
    app.ui
        .addins
        .iter()
        .filter_map(|text| addins::check_pack(text, None, &reg).ok())
        .map(|p| {
            let macros = p.macros.iter().map(|(n, _)| (addins::macro_key(&p.id, n), n.clone())).collect();
            (p.id, p.name, macros)
        })
        .collect()
}

/// Install a pack's text (checked by the engine) and keep it with the preferences.
pub(crate) fn install(app: &mut WordApp, text: &str, sha256: Option<&str>) -> Result<String, String> {
    let mut params = json!({"install": text});
    if let Some(h) = sha256 {
        params["sha256"] = json!(h);
    }
    let r = app.execute("addins.get", params)?;
    let id = r["installed"]["id"].as_str().unwrap_or("").to_string();
    let reg = app.session.registry.clone();
    app.ui.addins.retain(|t| addins::check_pack(t, None, &reg).map(|p| p.id != id).unwrap_or(false));
    app.ui.addins.push(text.to_string());
    Ok(id)
}

/// Install the kept packs again (at start-up, from the preferences); broken ones are dropped.
pub(crate) fn reinstall(app: &mut WordApp) {
    let kept = std::mem::take(&mut app.ui.addins);
    for text in kept.iter().take(100) {
        if let Err(e) = install(app, text, None) {
            log::warn!("add-in not installed: {e}");
        }
    }
}

fn remove(app: &mut WordApp, id: &str) {
    let _ = app.execute("addins.get", json!({"remove": id}));
    let reg = app.session.registry.clone();
    app.ui.addins.retain(|t| addins::check_pack(t, None, &reg).map(|p| p.id != id).unwrap_or(false));
}

/// Follow the downloads in flight.
pub(crate) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    let Some(store) = app.addins_store.as_mut() else { return };
    if let Some(p) = &store.catalog {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
        if let Some(answer) = p.poll() {
            store.catalog = None;
            match answer.and_then(|b| addins::parse_catalog(&b)) {
                Ok(entries) => {
                    store.message = entries.is_empty().then(|| tl!("No add-ins are listed yet.").to_string());
                    store.entries = entries;
                }
                Err(e) => store.message = Some(crate::i18n::fmt(tl!("Couldn't get the add-in list: {reason}"), &[("reason", &e)])),
            }
        }
    }
    let Some(store) = app.addins_store.as_mut() else { return };
    let done = match &store.adding {
        Some((entry, p)) => {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
            p.poll().map(|a| (entry.clone(), a))
        }
        None => None,
    };
    if let Some((entry, answer)) = done {
        store.adding = None;
        let result = answer
            .and_then(|bytes| String::from_utf8(bytes).map_err(|_| "the add-in isn't text".to_string()))
            .and_then(|text| install(app, &text, Some(&entry.sha256)));
        if let Some(store) = app.addins_store.as_mut() {
            store.message = Some(match result {
                Ok(_) => crate::i18n::fmt(tl!("{name} was added. Its macros are under My Add-ins and View › Macros."), &[("name", &entry.name)]),
                Err(e) => crate::i18n::fmt(tl!("Couldn't add {name}: {reason}"), &[("name", &entry.name), ("reason", &e)]),
            });
        }
    }
}

fn start_add(app: &mut WordApp, entry: Entry) {
    let url = format!("{}{}", addins::PACK_BASE, entry.file);
    let started = crate::net::start(app, FetchRequest::get(url, addins::MAX_PACK_BYTES));
    let Some(store) = app.addins_store.as_mut() else { return };
    match started {
        Ok(rx) => {
            store.message = Some(tl!("Downloading…").to_string());
            store.adding = Some((entry, Pending::new(rx)));
        }
        Err(e) => store.message = Some(crate::i18n::fmt(tl!("Couldn't add {name}: {reason}"), &[("name", &entry.name), ("reason", &e)])),
    }
}

/// The window.
pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    if app.addins_store.is_none() {
        return;
    }
    let t = Tokens::get(ctx);
    let mine_list = installed(app);
    let installed_ids: Vec<String> = mine_list.iter().map(|(id, _, _)| id.clone()).collect();
    let mut open = true;
    let mut add = None;
    let mut run = None;
    let mut drop_id = None;
    egui::Window::new(tl!("Add-ins"))
        .id(egui::Id::new("get-addins"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            let Some(store) = app.addins_store.as_mut() else { return };
            ui.set_width(560.0);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut store.mine, false, tl!("Store"));
                ui.selectable_value(&mut store.mine, true, tl!("My Add-ins"));
            });
            ui.separator();
            egui::ScrollArea::vertical().max_height(340.0).auto_shrink([false, true]).show(ui, |ui| {
                if !store.mine {
                    if store.catalog.is_some() {
                        ui.add(egui::Spinner::new());
                    }
                    for e in &store.entries {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_width(420.0);
                                ui.label(egui::RichText::new(&e.name).strong());
                                ui.label(&e.description);
                                ui.label(egui::RichText::new(format!("{} · {} · {}", e.author, e.license, e.version)).small().color(t.text_dim));
                            });
                            let have = installed_ids.contains(&e.id);
                            let label = if have { tl!("Added") } else { tl!("Add") };
                            if ui.add_enabled(!have && store.adding.is_none(), egui::Button::new(label)).clicked() {
                                add = Some(e.clone());
                            }
                        });
                        ui.separator();
                    }
                } else if mine_list.is_empty() {
                    ui.label(egui::RichText::new(tl!("No add-ins yet. Add one from the Store.")).color(t.text_dim));
                } else {
                    for (id, name, macros) in &mine_list {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(name).strong());
                            if ui.small_button(tl!("Remove")).clicked() {
                                drop_id = Some(id.clone());
                            }
                        });
                        for (key, shown) in macros {
                            ui.horizontal(|ui| {
                                ui.add_space(12.0);
                                if ui.small_button(tl!("Run")).clicked() {
                                    run = Some(key.clone());
                                }
                                ui.label(shown);
                            });
                        }
                        ui.separator();
                    }
                }
            });
            if let Some(m) = &store.message {
                ui.label(egui::RichText::new(m).color(t.text_dim));
            }
            ui.label(
                egui::RichText::new(tl!("Doc4Me add-ins are free packs of macros from Doc4Me's add-in catalog. They can only edit the document."))
                    .small()
                    .color(t.text_dim),
            );
        });
    if let Some(e) = add {
        start_add(app, e);
    }
    if let Some(key) = run
        && let Err(e) = app.run("tools.macros", json!({"run": key}))
    {
        app.status(e);
    }
    if let Some(id) = drop_id {
        remove(app, &id);
    }
    if !open {
        app.addins_store = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With a fake web: the catalog lists the packs, Add downloads, checks and installs one,
    /// it runs from My Add-ins, is kept for the next start and can be removed.
    #[test]
    fn an_add_in_is_added_run_kept_and_removed() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../addins");
        let catalog = std::fs::read(dir.join("catalog.json")).unwrap();
        let dir2 = dir.clone();
        let services = crate::Services {
            fetch: Some(Box::new(move |req: FetchRequest| {
                let (tx, rx) = std::sync::mpsc::channel();
                let body = if req.url == addins::CATALOG_URL {
                    Ok(catalog.clone())
                } else {
                    let file = req.url.trim_start_matches(addins::PACK_BASE);
                    std::fs::read(dir2.join(file)).map_err(|e| e.to_string())
                };
                let _ = tx.send(body);
                rx
            })),
            ..Default::default()
        };
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("a  b\n\nc")), services);
        let ctx = egui::Context::default();
        app.run("addins.get", json!({})).unwrap();
        poll(&mut app, &ctx);
        let entry = app.addins_store.as_ref().unwrap().entries.iter().find(|e| e.id == "tidy-text").cloned().expect("listed");
        start_add(&mut app, entry);
        poll(&mut app, &ctx);
        assert_eq!(installed(&app).len(), 1, "{:?}", app.addins_store.as_ref().unwrap().message);
        app.run("tools.macros", json!({"run": "tidy-text.Single spaces"})).unwrap();
        assert!(app.session.doc.plain_text(doc4me_doc::StoryRef::Body).starts_with("a b"));
        // Kept with the preferences, and installed again in a new window.
        let prefs = app.prefs();
        let mut again = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), crate::Services::default());
        again.apply_prefs(prefs);
        assert!(again.session.macros.contains_key("tidy-text.Remove empty paragraphs"));
        remove(&mut app, "tidy-text");
        assert!(installed(&app).is_empty() && !app.session.macros.keys().any(|k| k.starts_with("tidy-text.")));
    }

    #[test]
    fn a_tampered_pack_is_refused() {
        let services = crate::Services {
            fetch: Some(Box::new(|_req: FetchRequest| {
                let (tx, rx) = std::sync::mpsc::channel();
                let _ = tx.send(Ok(br#"{"id": "evil", "macros": [{"name": "m", "steps": [{"command": "format.bold"}]}]}"#.to_vec()));
                rx
            })),
            ..Default::default()
        };
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), services);
        let ctx = egui::Context::default();
        app.addins_store = Some(Store::default());
        let entry = Entry {
            id: "evil".into(),
            name: "Evil".into(),
            description: String::new(),
            author: String::new(),
            license: String::new(),
            version: String::new(),
            file: "evil.json".into(),
            sha256: "00".repeat(32),
        };
        start_add(&mut app, entry);
        poll(&mut app, &ctx);
        assert!(installed(&app).is_empty());
        assert!(app.addins_store.as_ref().unwrap().message.as_deref().is_some_and(|m| m.contains("SHA-256")));
    }
}
