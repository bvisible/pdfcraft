// //// Neoffice — added file (no upstream equivalent): a page that hosts the editor gets no control channel.
//! Upstream's web build offers `window.pdfcraft` (opt-in with `?control`), the page's command API over the UI control
//! channel: it runs any registry command by id and reads the screen. A page that hosts the editor owns its one document and
//! the way it is opened, saved and closed, and speaks the `pdfcraft:*` messages instead; the API there would let a script run
//! the commands that open, create and close other documents. So the channel is not served in such a window, whatever the
//! address of the page asks for: `attach_control` hands back a client whose calls are answered "the app has closed".
//! Everywhere else the channel works as upstream's.

use std::sync::{Arc, Mutex};

use egui_kittest::Harness;
use pdfcraft_ui_egui::PdfCraftApp;
use pdfcraft_ui_egui::control::{ControlClient, Reply};
use serde_json::json;

/// A window with the control channel attached, as the web build does for `?control`, and the client of that channel.
fn with_control(hosted: bool) -> (Harness<'static, PdfCraftApp>, ControlClient) {
    let slot: Arc<Mutex<Option<ControlClient>>> = Arc::default();
    let s = slot.clone();
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.host_owns_documents = hosted;
        *s.lock().unwrap() = Some(app.attach_control(&cc.egui_ctx));
        app
    });
    h.run_steps(4);
    let client = slot.lock().unwrap().take().unwrap();
    (h, client)
}

/// The answer to `method`, after the window had frames to serve it (`None` when no answer ever came).
fn answer(h: &mut Harness<'static, PdfCraftApp>, client: &ControlClient, method: &str, params: serde_json::Value) -> Option<Reply> {
    let rx = client.send(method, params);
    for _ in 0..20 {
        h.run_steps(2);
        if let Ok(reply) = rx.try_recv() {
            return Some(reply);
        }
    }
    None
}

#[test]
fn the_control_channel_answers_a_window_no_page_hosts() {
    let (mut h, client) = with_control(false);
    let reply = answer(&mut h, &client, "ui.commands", json!({})).expect("the channel answers");
    assert!(reply.is_ok(), "ui.commands: {reply:?}");
    let reply = answer(&mut h, &client, "ui.state", json!({})).expect("the channel answers");
    assert!(reply.is_ok(), "ui.state: {reply:?}");
}

#[test]
fn a_page_that_hosts_the_editor_gets_no_control_channel() {
    let (mut h, client) = with_control(true);
    assert!(!h.state().host_allows_page_api(), "the question the web build asks before it installs window.pdfcraft");
    let before = h.state().views.len();
    // What the page API's `commands`, `execute`, `docInspect` and `screenshot` send, and a command that would open a document.
    for (method, params) in [
        ("ui.commands", json!({})),
        ("ui.state", json!({})),
        ("ui.inspect", json!({})),
        ("ui.screenshot", json!({})),
        ("ui.command", json!({ "id": "create.blank" })),
        ("ui.command", json!({ "id": "file.open" })),
        ("ui.command", json!({ "id": "page.combine" })),
    ] {
        let reply = answer(&mut h, &client, method, params.clone()).expect("an answer comes at once: the channel is not served");
        assert_eq!(reply, Err("the app has closed".to_string()), "{method} {params}: a hosted window serves nothing");
    }
    assert_eq!(h.state().views.len(), before, "no document came in");
}

/// The web build's `main.rs` (wasm only, so no test of ours can run it) installs `window.pdfcraft` only behind the question
/// the tests above answer; a change that put the call elsewhere would get no answer from them, so this reads the source.
#[test]
fn the_web_build_installs_the_page_api_only_where_the_host_allows_it() {
    let main = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../apps/pdfcraft-web/src/main.rs")).expect("the web build's main.rs");
    let installs: Vec<usize> = main.match_indices("api::install(").map(|(at, _)| at).collect();
    assert_eq!(installs.len(), 1, "one place installs window.pdfcraft");
    let before = &main[..installs[0]];
    let guard = before.rfind("host_allows_page_api()").expect("a question before the install");
    assert!(installs[0] - guard < 200, "the question is the one that guards the install, not another far above it");
    assert!(
        main.contains("app.host_owns_documents = true"),
        "the web build says a page hosts the editor before it asks (`?host=parent` sets host_owns_documents)"
    );
    assert!(main.find("app.host_owns_documents = true") < installs.first().copied(), "…and says it before the install");
}
