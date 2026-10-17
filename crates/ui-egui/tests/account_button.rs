// //// Neoffice — added file (no upstream equivalent): the tab strip's account button.
//! The account button at the right end of the tab strip, the window's top row: the one WordCraft draws at the end of
//! its title bar, the initials of the person in a round badge of the accent colour, and their name, nothing else, as
//! the hover text. The name is the one that signs comments, which a hosting page passes as `?author=`. Without a name,
//! or with the default PdfCraft gives a user nobody named, there is no button. Like WordCraft's, the button has no
//! accessible label: it is found here by its badge.

use egui::epaint::CircleShape;
use egui::{Color32, Rect, Shape, Vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::PdfCraftApp;
use pdfcraft_ui_egui::theme::{ThemeKind, Tokens};

const WIDTH: f32 = 1400.0;

fn harness(author: Option<&str>, dark: bool) -> Harness<'static, PdfCraftApp> {
    let author = author.map(str::to_string);
    let mut h = Harness::builder().with_size(egui::vec2(WIDTH, 900.0)).with_step_dt(1.0 / 60.0).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.set_option("theme", if dark { "dark" } else { "light" }).unwrap();
        if let Some(author) = author {
            app.comment_prefs.author = author;
        }
        app
    });
    // Fonts install on frame 1 and apply on frame 2.
    h.run_steps(6);
    h
}

/// The theme button, the last control of the strip before ours.
fn theme_button(h: &Harness<'static, PdfCraftApp>, dark: bool) -> Rect {
    h.get_by_label(if dark { "Display theme: Dark gray" } else { "Display theme: Light gray" }).rect()
}

/// The badge: a circle in the accent colour, level with the theme button.
fn badge(h: &Harness<'static, PdfCraftApp>, accent: Color32, row: Rect) -> Option<CircleShape> {
    h.output().shapes.iter().find_map(|s| match &s.shape {
        Shape::Circle(c) if c.fill == accent && row.y_range().contains(c.center.y) => Some(*c),
        _ => None,
    })
}

/// The square the badge fills (its circle keeps a point of margin).
fn square(badge: &CircleShape) -> Rect {
    Rect::from_center_size(badge.center, Vec2::splat(2.0 * (badge.radius + 1.0)))
}

/// The painted text `text`: its rectangle, font size and colour.
fn text(h: &Harness<'static, PdfCraftApp>, text: &str) -> Option<(Rect, f32, Color32)> {
    h.output().shapes.iter().find_map(|s| match &s.shape {
        Shape::Text(t) if t.galley.job.text == text => {
            let format = &t.galley.job.sections.first()?.format;
            Some((t.galley.rect.translate(t.pos.to_vec2()), format.font_id.size, format.color))
        }
        _ => None,
    })
}

/// WCAG contrast ratio of two opaque colours.
fn contrast(a: Color32, b: Color32) -> f32 {
    fn luminance(c: Color32) -> f32 {
        let lin = |v: u8| {
            let v = f32::from(v) / 255.0;
            if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
    }
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[test]
fn the_tab_strip_ends_with_the_initials_of_the_person_named() {
    for dark in [false, true] {
        let tokens = Tokens::for_kind(if dark { ThemeKind::Dark } else { ThemeKind::Light });
        let h = harness(Some("Jane Doe"), dark);
        let theme = theme_button(&h, dark);
        // The badge, in the accent colour, fills a square of the strip's control height.
        let circle = badge(&h, tokens.accent, theme).expect("the badge is painted");
        let button = square(&circle);
        assert_eq!(button.height(), theme.height(), "dark={dark}: {button:?}");
        // Right of the theme button, at the strip's 10 point margin, level with the other controls.
        assert!(theme.right() <= button.left(), "dark={dark}: after the theme button {theme:?}: {button:?}");
        assert_eq!(WIDTH - button.right(), 10.0, "dark={dark}: the last control of the strip");
        assert!((button.center().y - theme.center().y).abs() <= 1.0, "dark={dark}: level with the other controls");
        // The initials sit on it, small and legible.
        let (at, size, color) = text(&h, "JD").expect("the initials are painted");
        assert!(button.contains(at.center()), "dark={dark}: {at:?} in {button:?}");
        assert_eq!(size, 10.5, "dark={dark}");
        let ratio = contrast(color, tokens.accent);
        assert!(ratio >= 4.5, "dark={dark}: the initials read at {ratio:.2}:1");
    }
}

#[test]
fn hovering_the_button_shows_the_name_and_nothing_else() {
    let mut h = harness(Some("Jane Doe"), false);
    let circle = badge(&h, Tokens::for_kind(ThemeKind::Light).accent, theme_button(&h, false)).expect("the badge is painted");
    assert!(h.query_by_label_contains("Jane Doe").is_none(), "the name is not shown before the pointer comes");
    h.hover_at(circle.center);
    // Past egui's tooltip delay (half a second).
    h.run_steps(60);
    assert!(text(&h, "Jane Doe").is_some(), "the tooltip is the name itself");
    assert!(h.query_by_role_and_label(egui::accesskit::Role::Label, "Jane Doe").is_some(), "the tooltip is the name itself");
}

#[test]
fn no_button_without_a_known_name() {
    let accent = Tokens::for_kind(ThemeKind::Light).accent;
    // No name (PdfCraft's default: the login name, "Guest" on the web), an empty one, a blank one.
    for author in [None, Some(""), Some("   ")] {
        let h = harness(author, false);
        assert!(badge(&h, accent, theme_button(&h, false)).is_none(), "{author:?}: no badge");
    }
    // The name is read each frame: once a name comes, the button does.
    let mut h = harness(None, false);
    h.state_mut().comment_prefs.author = "Ada Lovelace".into();
    h.run_steps(2);
    assert!(badge(&h, accent, theme_button(&h, false)).is_some());
    assert!(text(&h, "AL").is_some());
}

/// A one-page PDF to have documents open.
const PAGE: &[u8] = b"%PDF-1.7
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 300 400] >> endobj
trailer << /Root 1 0 R >>
%%EOF";

#[test]
fn the_tabs_and_open_keep_clear_of_the_button() {
    // Many documents: the tabs shrink, then scroll, in the room the strip leaves them. That room leaves out the
    // controls on the right, the account button included, so the Open button after the tabs ends before the first of
    // them (Discord's) and the account button stays at the end of the strip.
    let mut h = harness(Some("Jane Doe"), false);
    for i in 0..16 {
        h.state_mut().open_bytes(&format!("A document with a rather long name {i}.pdf"), None, PAGE.to_vec()).unwrap();
    }
    h.run_steps(6);
    let open = h.get_by_label("Open").rect();
    let discord = h.get_by_label("Discord").rect();
    assert!(open.right() <= discord.left(), "Open {open:?} runs into the controls {discord:?}");
    let circle = badge(&h, Tokens::for_kind(ThemeKind::Light).accent, theme_button(&h, false)).expect("the badge is painted");
    assert_eq!(WIDTH - square(&circle).right(), 10.0, "the last control of the strip");
}
