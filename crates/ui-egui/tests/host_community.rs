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
