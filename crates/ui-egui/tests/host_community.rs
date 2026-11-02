// //// Neoffice — added file (no upstream equivalent): what a hosting page hides.
//! A page that frames the editor (the Drive of a Neoffice instance, Neodesk's window) has its own community: upstream's
//! links (Discord) are not shown, and the command that opens them does nothing. Everywhere else they are as before.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::PdfCraftApp;

fn harness(hidden: bool) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.hide_community = hidden;
        app
    });
    // Fonts install on frame 1 and apply on frame 2.
    h.run_steps(4);
    h
}

#[test]
fn the_community_links_are_there_unless_the_host_hides_them() {
    let shown = harness(false);
    shown.get_by_label("Discord");
    shown.get_by_label_contains("Join the ArtCraft community");

    let hidden = harness(true);
    assert!(hidden.query_by_label("Discord").is_none(), "the title bar's button");
    assert!(hidden.query_by_label_contains("Join the ArtCraft community").is_none(), "the Home page's card");
    // The rest of the Home page is where it was.
    hidden.get_by_label_contains("Welcome to PdfCraft");
    hidden.get_by_label("Open file");
}

#[test]
fn the_discord_command_opens_nothing_when_the_host_hides_the_community_links() {
    let mut hidden = harness(true);
    hidden.state_mut().execute("help.discord");
    assert!(hidden.state().last_opened_url.is_none());

    let mut shown = harness(false);
    shown.state_mut().execute("help.discord");
    assert_eq!(shown.state().last_opened_url.as_deref(), Some("https://discord.gg/artcraft"));
}

/// Upstream's update check opens upstream's releases page in the browser build; a hosting page serves the editor's
/// version, so the command is neither offered nor run there.
#[test]
fn the_update_check_is_hidden_when_the_host_hides_the_community_links() {
    let mut hidden = harness(true);
    assert!(!hidden.state_mut().execute("help.check_updates"), "help.check_updates ran");
    assert!(hidden.state().last_opened_url.is_none(), "upstream's releases page");

    // Elsewhere it runs: the native app opens its Updates dialog, the browser build upstream's releases page.
    let mut shown = harness(false);
    assert!(shown.state_mut().execute("help.check_updates"));
    assert_eq!(shown.state().last_opened_url.is_some(), cfg!(target_arch = "wasm32"), "the releases page opens in the browser build only");
}

/// The window's menu bar (upstream 0.17) has a Help menu with upstream's links; a hosting page has its own community, so
/// those links are not in its menus, nor is the update check. The shortcuts, the About item and the rest of Help stay.
#[test]
fn the_help_menu_lists_no_link_of_upstream_when_the_host_hides_the_community_links() {
    let commands =
        |app: &PdfCraftApp| -> Vec<String> { app.menu_bar(&egui::Context::default()).items().iter().map(|item| item.command.id.clone()).collect() };
    const UPSTREAM: [&str; 5] = ["help.discord", "help.app_page", "help.github", "help.website", "help.check_updates"];
    let shown = harness(false);
    let there = commands(shown.state());
    for id in UPSTREAM {
        assert!(there.iter().any(|c| c == id), "{id} is in the Help menu elsewhere");
    }
    let hidden = harness(true);
    let here = commands(hidden.state());
    for id in UPSTREAM {
        assert!(!here.iter().any(|c| c == id), "{id} is in the Help menu of a hosted window");
    }
    for id in ["help.shortcuts", "help.about"] {
        assert!(here.iter().any(|c| c == id), "{id} stays");
    }
}
