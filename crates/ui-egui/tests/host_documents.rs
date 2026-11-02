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

#[test]
fn the_document_of_the_host_stays_in_its_tab() {
    let mut shown = with_a_document(false);
    shown.state_mut().request_close_tab(0);
    shown.run_steps(2);
    assert_eq!(shown.state().views.len(), 0, "elsewhere a tab closes");

    let mut hosted = with_a_document(true);
    assert!(!hosted.state_mut().execute("file.close"), "file.close ran");
    assert!(!hosted.state_mut().execute("file.close_all"), "file.close_all ran");
    hosted.state_mut().request_close_tab(0);
    hosted.run_steps(2);
    assert_eq!(hosted.state().views.len(), 1, "the page's document is still open");
    let said = hosted.state().toast.as_ref().map(|(m, _)| m.clone()).unwrap_or_default();
    assert!(said.contains("closed from the page"), "the person is told: {said:?}");
}

/// What the editor makes from the page's document opens in a tab of its own elsewhere: a comment summary, a saved
/// revision, a signed version, an attachment, the pages extracted into one document. A page that opens the documents gets
/// none of them (the tab could not be closed, and its Save would store a file the page never asked for), and Delete pages
/// after extracting takes nothing from the page's document.
#[test]
fn a_document_made_from_the_page_s_one_is_refused_when_the_host_opens_the_documents() {
    let mut shown = with_a_document(false);
    shown.state_mut().summarize_comments();
    shown.run_steps(2);
    assert_eq!(shown.state().views.len(), 2, "elsewhere the comment summary opens in a tab of its own");

    let mut hosted = with_a_document(true);
    hosted.state_mut().summarize_comments();
    hosted.state_mut().open_revision(1);
    hosted.state_mut().view_signed_version(PAGE.len(), 1);
    hosted.state_mut().extract_draft.delete = true;
    hosted.state_mut().extract_selection();
    hosted.run_steps(2);
    assert_eq!(hosted.state().views.len(), 1, "no other document came in");
    let id = hosted.state().views[0].id;
    assert_eq!(hosted.state().session.get(id).map(|d| d.info.pages.len()), Some(1), "the page's document kept its page");
    let said = hosted.state().toast.as_ref().map(|(m, _)| m.clone()).unwrap_or_default();
    assert!(said.contains("hosts the editor"), "the person is told: {said:?}");
}

/// Every command id in the window's menu bar (the one definition the native macOS bar and the in-window bar are laid out from).
fn menu_commands(app: &PdfCraftApp) -> Vec<String> {
    app.menu_bar(&egui::Context::default()).items().iter().map(|item| item.command.id.clone()).collect()
}

/// The titles of the submenus of the menu bar, at any depth.
fn menu_titles(app: &PdfCraftApp) -> Vec<String> {
    fn walk(nodes: &[craft_menu::Node], out: &mut Vec<String>) {
        for node in nodes {
            if let craft_menu::Node::Submenu { label, children, .. } = node {
                out.push(label.clone());
                walk(children, out);
            }
        }
    }
    let bar = app.menu_bar(&egui::Context::default());
    let mut titles = Vec::new();
    for menu in &bar.menus {
        walk(&menu.children, &mut titles);
    }
    titles
}

/// The window's menus (upstream 0.17) offer nothing that opens or creates a document, nothing that only answers "the page
/// opens the documents", for a host that opens the documents; elsewhere they list all of it.
#[test]
fn the_menu_bar_offers_no_command_that_opens_or_creates_a_document_to_a_host_that_opens_the_documents() {
    const OPENING: [&str; 15] = [
        "file.open",
        "file.open_recent",
        "file.clear_recent",
        "file.pin_folder",
        "create.blank",
        "create.file",
        "create.multiple",
        "create.multiple_separate",
        "create.images",
        "create.clipboard",
        "page.combine",
        "comment.summarize",
        "doc.compare",
        "file.close",
        "file.close_all",
    ];
    let shown = harness(false);
    let there = menu_commands(shown.state());
    for id in OPENING {
        // The recent files are a submenu whose items carry an index, and "no recent files" stands for them before any;
        // Summarize comments and Compare files are in the menus only through View ▸ Tools.
        if matches!(id, "file.open_recent" | "file.clear_recent" | "comment.summarize" | "doc.compare") {
            continue;
        }
        assert!(there.iter().any(|c| c == id), "{id} is in the menus of a window that opens its documents");
    }
    assert!(menu_titles(shown.state()).iter().any(|t| t == "Create"), "the Create submenu");

    let hosted = harness(true);
    let here = menu_commands(hosted.state());
    for id in OPENING {
        assert!(!here.iter().any(|c| c == id), "{id} is in the menus of a window the page opens the documents of");
    }
    let titles = menu_titles(hosted.state());
    for title in ["Create", "Open Recent"] {
        assert!(!titles.iter().any(|t| t == title), "the {title} submenu");
    }
    // The menus are otherwise where they were: saving, printing, the view and the pages.
    for id in ["file.save", "file.save_as", "print.dialog", "view.palette", "page.rotate", "edit.find"] {
        assert!(here.iter().any(|c| c == id), "{id} stays");
    }
}

/// View ▸ Tools lists the groups of the All tools list, which has no Create, Combine or Compare for such a host; nor does
/// View ▸ Side panels offer the Compare panel.
#[test]
fn the_menu_bar_lists_no_tool_that_opens_another_document_to_a_host_that_opens_the_documents() {
    let shown = harness(false);
    let titles = menu_titles(shown.state());
    let group = |name: &str| pdfcraft_engine::catalog::group(name).map(|g| g.label.to_owned()).unwrap_or_default();
    // Create and Combine files have no group of their own there when every command of theirs is in the File menu already.
    assert!(titles.iter().any(|t| *t == group("compare")), "the Compare tool group is in View ▸ Tools");
    assert!(menu_commands(shown.state()).iter().any(|c| c == "doc.compare"), "and its command");
    let hosted = harness(true);
    let titles = menu_titles(hosted.state());
    for id in ["create", "combine", "compare"] {
        assert!(!titles.iter().any(|t| *t == group(id)), "the {id} tool group is in View ▸ Tools of a hosted window");
    }
    // The Compare panel is a side panel of View ▸ Side panels, a command with the panel's name as its parameter.
    let compare_panel = |app: &PdfCraftApp| {
        app.menu_bar(&egui::Context::default())
            .items()
            .iter()
            .any(|item| item.command.id == "view.panel" && item.command.params.get("panel").and_then(|p| p.as_str()) == Some("compare"))
    };
    assert!(compare_panel(shown.state()), "the Compare panel is in View ▸ Side panels elsewhere");
    assert!(!compare_panel(hosted.state()), "and not in the window the page opens the documents of");
}
