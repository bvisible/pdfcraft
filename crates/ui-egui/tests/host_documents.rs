// //// Neoffice — added file (no upstream equivalent): a page that frames the editor opens the documents.
//! A hosting page (the Drive of a Neoffice instance, Neodesk's window) gives the editor the document to edit and
//! takes what it saves. The editor must not offer to open another one beside it: its save would reach the page under
//! a name the page did not ask for. The Home page has no Open card and no Recent list, and the commands that open a
//! file refuse. Everywhere else they are as before.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::PdfCraftApp;

fn harness(hosted: bool) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.host_owns_documents = hosted;
        app
    });
    // Fonts install on frame 1 and apply on frame 2.
    h.run_steps(4);
    h
}

#[test]
fn the_home_page_offers_no_open_and_no_recent_files_to_a_host_that_opens_the_documents() {
    let shown = harness(false);
    shown.get_by_label("Open file");
    shown.get_by_label_contains("Recent");

    let hosted = harness(true);
    hosted.get_by_label_contains("Welcome to PdfCraft");
    assert!(hosted.query_by_label("Open file").is_none(), "the Open card");
    assert!(hosted.query_by_label("Recent").is_none(), "the Recent list's heading");
    assert!(hosted.query_by_label_contains("appear here").is_none(), "the hint of the Recent list");
    // The tools are where they were.
    assert!(hosted.query_all_by_label_contains("Organize").count() > 0, "the recommended tools");
}

#[test]
fn opening_a_file_is_refused_when_the_host_opens_the_documents() {
    let mut hosted = harness(true);
    let before = hosted.state().views.len();
    hosted.state_mut().execute("file.open");
    hosted.state_mut().open_dialog();
    hosted.state_mut().open_recent("/tmp/another.pdf");
    hosted.run_steps(2);
    assert_eq!(hosted.state().views.len(), before, "no document came in");
    let said = hosted.state().toast.as_ref().map(|(m, _)| m.clone()).unwrap_or_default();
    assert!(said.contains("hosts the editor"), "the person is told: {said:?}");
}
