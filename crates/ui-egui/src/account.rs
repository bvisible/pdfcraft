// //// Neoffice — added file (no upstream equivalent): the account button of the tab strip.

//! The account button at the right end of the tab strip (the window's top row), the one WordCraft
//! draws at the end of its title bar: a round badge in the accent colour with the initials of the
//! person, and their name as the hover text. The name is the one comments are signed with
//! (`CommentPrefs::author`), which a hosting page (NeoDesk, the Drive of the Suite) sets from
//! `?author=`. The button shows only when a name is known: not while the name is blank or the
//! default PdfCraft gives a user nobody named (the login name, "Guest" on the web).

use std::sync::LazyLock;

use egui::{Align2, Color32, Sense, Ui, vec2};

use crate::theme::{self, Tokens};

/// The name PdfCraft starts with when nobody named the user.
static PLACEHOLDER: LazyLock<String> = LazyLock::new(|| crate::comments::CommentPrefs::default().author);

/// The person's name when one is known: not blank, and not PdfCraft's default.
pub fn known_name(name: &str) -> Option<&str> {
    let name = name.trim();
    (!name.is_empty() && name != PLACEHOLDER.as_str()).then_some(name)
}

/// Up to two initials: the first letters of the name's words, as WordCraft takes them.
pub fn initials(name: &str) -> String {
    name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect()
}

/// The room the button takes in a row whose items are `gap` apart: its square and one gap, or
/// nothing when no name is known.
pub fn width(name: &str, size: f32, gap: f32) -> f32 {
    if known_name(name).is_some() { size + gap } else { 0.0 }
}

/// The colour of the initials on the accent: of white and the theme's dark, the one that reads
/// better there. White on the light theme (5.4:1, the text of the primary buttons there); on the
/// dark theme, where white reads at 3.3:1, the strip's own colour (5.2:1).
fn ink(t: &Tokens) -> Color32 {
    if t.dark() { t.titlebar } else { Color32::WHITE }
}

/// Paints the button, `size` square, with the name as its hover text. Nothing is drawn when no
/// name is known. Like WordCraft's, the button has no accessible label of its own.
pub fn button(ui: &mut Ui, name: &str, size: f32) {
    let Some(name) = known_name(name) else { return };
    let t = Tokens::get(ui.ctx());
    let (r, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    ui.painter().circle_filled(r.center(), size / 2.0 - 1.0, t.accent);
    ui.painter().text(r.center(), Align2::CENTER_CENTER, initials(name), theme::semibold(10.5), ink(&t));
    resp.on_hover_text(name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::ThemeKind;

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
    fn the_initials_read_on_the_accent_in_both_themes() {
        for t in [Tokens::for_kind(ThemeKind::Light), Tokens::for_kind(ThemeKind::Dark)] {
            let r = contrast(ink(&t), t.accent);
            assert!(r >= 4.5, "dark={}: initials on the accent at {r:.2}:1", t.dark());
            assert!(r >= contrast(Color32::WHITE, t.accent), "dark={}: white would read better than {r:.2}:1", t.dark());
        }
    }

    #[test]
    fn a_name_is_known_unless_blank_or_the_default() {
        assert_eq!(known_name("  Jane Doe "), Some("Jane Doe"));
        assert_eq!(known_name(""), None);
        assert_eq!(known_name("   "), None);
        assert_eq!(known_name(&crate::comments::CommentPrefs::default().author), None, "the default names nobody");
    }

    #[test]
    fn initials_are_the_first_letters_of_the_first_two_words() {
        assert_eq!(initials("Jane Doe"), "JD");
        assert_eq!(initials("Jane Mary Doe"), "JM");
        assert_eq!(initials("Jane"), "J");
        assert_eq!(initials("Élodie Ünal"), "ÉÜ");
        assert_eq!(initials("Jean-Luc Picard"), "JP");
        assert_eq!(initials(""), "");
    }

    #[test]
    fn the_button_takes_room_only_when_it_shows() {
        assert_eq!(width("Jane Doe", 28.0, 4.0), 32.0);
        assert_eq!(width(" ", 28.0, 4.0), 0.0);
        assert_eq!(width(&crate::comments::CommentPrefs::default().author, 28.0, 4.0), 0.0);
    }
}
