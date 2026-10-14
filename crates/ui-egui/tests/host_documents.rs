// //// Neoffice — added file (no upstream equivalent): a page that frames the editor opens the documents.
//! A hosting page (the Drive of a Neoffice instance, Neodesk's window) gives the editor the document to edit and
//! takes what it saves. The editor must not offer to open another one beside it: its save would reach the page under
//! a name the page did not ask for. The Home page has no Open card and no Recent list, and the commands that open a
//! file refuse, and so do the ones that create another document (blank, from images, from the clipboard, combined files):
//! none is offered by the tab strip, the menus, the palette or the tools list. Everywhere else they are as before.

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

/// A one-page PDF to have a document open.
const PAGE: &[u8] = b"%PDF-1.7
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 300 400] >> endobj
trailer << /Root 1 0 R >>
%%EOF";

fn with_a_document(hosted: bool) -> Harness<'static, PdfCraftApp> {
    let mut h = harness(hosted);
    h.state_mut().open_bytes("page.pdf", None, PAGE.to_vec()).unwrap();
    h.run_steps(4);
    h
}

#[test]
fn the_tab_strip_and_the_tools_list_offer_no_other_document_to_a_host_that_opens_the_documents() {
    let shown = with_a_document(false);
    shown.get_by_label("Open");
    assert!(shown.query_all_by_label("Create a PDF").count() > 0, "the Create a PDF tool");
    assert!(shown.query_all_by_label("Combine files").count() > 0, "the Combine files tool");

    let hosted = with_a_document(true);
    hosted.get_by_label_contains("page.pdf");
    assert!(hosted.query_by_label("Open").is_none(), "the strip's Open button");
    assert!(hosted.query_all_by_label("Create a PDF").next().is_none(), "the Create a PDF tool");
    assert!(hosted.query_all_by_label("Combine files").next().is_none(), "the Combine files tool");
    // The tools that work on the document are where they were.
    assert!(hosted.query_all_by_label("Organize pages").count() > 0, "the tools of the document");
}

#[test]
fn creating_a_document_is_refused_when_the_host_opens_the_documents() {
    let mut shown = with_a_document(false);
    let before = shown.state().views.len();
    shown.state_mut().execute("create.blank");
    shown.run_steps(2);
    assert_eq!(shown.state().views.len(), before + 1, "elsewhere a blank PDF opens in a tab of its own");
    shown.state_mut().set_option("tool", "create").unwrap();
    shown.run_steps(3);
    assert!(shown.query_all_by_label("Multiple files").count() > 0, "elsewhere the Create a PDF tool lists its sources");

    let mut hosted = with_a_document(true);
    let before = hosted.state().views.len();
    for id in ["create.blank", "create.file", "create.images", "create.clipboard", "create.multiple", "create.multiple_separate", "page.combine"] {
        assert!(!hosted.state_mut().execute(id), "{id} ran");
        hosted.state_mut().toast = None;
    }
    // The control channel's way to a tool, too.
    hosted.state_mut().set_option("tool", "create").unwrap();
    hosted.run_steps(3);
    assert!(hosted.query_all_by_label("Multiple files").next().is_none(), "the Create a PDF tool opened");
    assert_eq!(hosted.state().views.len(), before, "no other document came in");
    assert!(!hosted.state().combine_showing(), "no Combine tab");
    // Said once more, to read the answer: it is the page's own words.
    hosted.state_mut().execute("create.blank");
    let said = hosted.state().toast.as_ref().map(|(m, _)| m.clone()).unwrap_or_default();
    assert!(said.contains("hosts the editor"), "the person is told: {said:?}");
}

#[test]
fn the_palette_finds_no_command_that_opens_or_creates_a_document_for_a_host_that_opens_the_documents() {
    for (hosted, query) in [(false, "blank"), (true, "blank"), (false, "open recent"), (true, "open recent")] {
        let mut h = with_a_document(hosted);
        h.state_mut().palette_open = true;
        h.state_mut().palette_query = query.to_string();
        h.run_steps(3);
        let found = h.query_all_by_label_contains(if query == "blank" { "New blank PDF" } else { "Open Recent" }).count();
        assert_eq!(found > 0, !hosted, "{query:?}, hosted {hosted}: {found} hit(s)");
    }
}
