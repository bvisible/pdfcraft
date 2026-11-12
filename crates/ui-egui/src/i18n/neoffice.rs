// //// Neoffice — added file (no upstream equivalent)
//! The strings Neoffice added to PdfCraft (the Nora assistant pane, its switch in Preferences and its
//! command), translated here, outside the craft-i18n catalogs. Upstream moved its catalogs out of the
//! repository in 0.19 (storytold/craft-i18n, embedded from `CRAFT_I18N_DIR`), so a row of ours has no
//! place there; the rows we had written for the 34 languages are kept in `neoffice/<code>.rs`, one table
//! of (English source, translation) per language, as WordCraft keeps its own in Rust.
//!
//! [`super::tr`] asks here only when the language's catalogs hold no entry for a string, so a catalog
//! that gains the same English string wins and an upstream merge never touches these. Only plain rows
//! (no context, no `@id`, no plural form) exist: nothing of ours needs one.
//!
//! A language without a table here (English) shows the English string. The font tests read these rows
//! through [`super::Lang::catalog_texts`], as they read the catalogs, in the catalogs' own text form.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::{LANGUAGES, Lang};

mod ar;
mod bg;
mod ca;
mod ckb;
mod cs;
mod de;
mod el;
mod es;
mod fa;
mod fi;
mod fr;
mod hi;
mod hu;
mod id;
mod it;
mod ja;
mod kk;
mod ko;
mod lo;
mod lt;
mod nb;
mod pl;
mod pt_br;
mod ro;
mod ru;
mod sl;
mod sq;
mod te;
mod th;
mod tr;
mod uk;
mod vi;
mod zh_hans;
mod zh_hant;

/// The rows of the language `code`, `None` for English and for a language of ours without a table.
fn table(code: &str) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match code.to_ascii_lowercase().as_str() {
        "ar" => ar::ROWS,
        "bg" => bg::ROWS,
        "ca" => ca::ROWS,
        "ckb" => ckb::ROWS,
        "cs" => cs::ROWS,
        "de" => de::ROWS,
        "el" => el::ROWS,
        "es" => es::ROWS,
        "fa" => fa::ROWS,
        "fi" => fi::ROWS,
        "fr" => fr::ROWS,
        "hi" => hi::ROWS,
        "hu" => hu::ROWS,
        "id" => id::ROWS,
        "it" => it::ROWS,
        "ja" => ja::ROWS,
        "kk" => kk::ROWS,
        "ko" => ko::ROWS,
        "lo" => lo::ROWS,
        "lt" => lt::ROWS,
        "nb" => nb::ROWS,
        "pl" => pl::ROWS,
        "pt-br" => pt_br::ROWS,
        "ro" => ro::ROWS,
        "ru" => ru::ROWS,
        "sl" => sl::ROWS,
        "sq" => sq::ROWS,
        "te" => te::ROWS,
        "th" => th::ROWS,
        "tr" => tr::ROWS,
        "uk" => uk::ROWS,
        "vi" => vi::ROWS,
        "zh-hans" => zh_hans::ROWS,
        "zh-hant" => zh_hant::ROWS,
        _ => return None,
    })
}

/// Per language of [`LANGUAGES`], its rows by English source, built on first use.
static ROWS: [OnceLock<HashMap<&'static str, &'static str>>; super::LANGUAGE_COUNT] = [const { OnceLock::new() }; super::LANGUAGE_COUNT];

/// Per language of [`LANGUAGES`], its rows in the catalogs' text form (`\tsource\ttranslation` lines), built on first use.
static TEXTS: [OnceLock<String>; super::LANGUAGE_COUNT] = [const { OnceLock::new() }; super::LANGUAGE_COUNT];

/// The rows of `code` in the catalogs' text form, for the checks that the fonts draw every translation.
pub(super) fn text(code: &str) -> Option<&'static str> {
    let rows = table(code)?;
    let slot = LANGUAGES.iter().position(|l| l.code.eq_ignore_ascii_case(code)).and_then(|i| TEXTS.get(i))?;
    Some(slot.get_or_init(|| rows.iter().map(|(source, translation)| format!("\t{source}\t{translation}\n")).collect()).as_str())
}

/// `source`'s translation in `lang`, `None` when we have none. A test fixture's language owns its
/// rows (it is not one of ours), so it never reads these.
pub(super) fn plain(lang: Lang, source: &str) -> Option<&'static str> {
    if lang.catalogs.is_some() {
        return None;
    }
    lookup(lang.code(), source)
}

/// `source`'s translation in the language `code`, `None` when we have none.
fn lookup(code: &str, source: &str) -> Option<&'static str> {
    let rows = table(code)?;
    let slot = LANGUAGES.iter().position(|l| l.code == code).and_then(|i| ROWS.get(i))?;
    // A source that appears twice keeps its first row.
    let by_source = slot.get_or_init(|| rows.iter().rev().map(|(source, translation)| (*source, *translation)).collect());
    by_source.get(source).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every language we offer, English apart, has a table, and no table is for a language we don't offer.
    #[test]
    fn every_language_has_its_rows() {
        for info in &LANGUAGES {
            assert_eq!(table(info.code).is_some(), info.code != "en", "{}", info.code);
        }
        assert!(table("x-unknown").is_none());
    }

    /// The catalogs' text form reads one row to a line, so no source or translation may hold a tab or a line break.
    #[test]
    fn no_row_holds_a_tab_or_a_line_break() {
        for info in LANGUAGES.iter().filter(|l| l.code != "en") {
            for (source, translation) in table(info.code).expect("a table") {
                assert!(!source.contains(['\t', '\n']) && !translation.contains(['\t', '\n']), "{}: {source:?}", info.code);
                assert!(!source.is_empty() && !translation.is_empty(), "{}: an empty row {source:?}", info.code);
            }
        }
    }

    /// Our 34 languages all say the same things, with the same `{placeholders}`, and no source twice.
    #[test]
    fn every_language_translates_the_same_sources_and_keeps_their_placeholders() {
        fn holes(s: &str) -> Vec<&str> {
            let mut found: Vec<&str> = s.split('{').skip(1).filter_map(|p| p.split_once('}').map(|(name, _)| name)).collect();
            found.sort_unstable();
            found
        }
        let french: Vec<&str> = table("fr").expect("French rows").iter().map(|(source, _)| *source).collect();
        assert!(french.len() > 70, "the French rows: {}", french.len());
        for info in LANGUAGES.iter().filter(|l| l.code != "en") {
            let rows = table(info.code).expect("a table");
            let mut seen = std::collections::HashSet::new();
            for (source, translation) in rows {
                assert!(seen.insert(*source), "{}: {source:?} twice", info.code);
                assert_eq!(holes(source), holes(translation), "{}: placeholders of {source:?}", info.code);
                assert!(french.contains(source), "{} translates a source French lacks: {source:?}", info.code);
            }
        }
    }

    /// A row of ours is found, in the language asked for and not in another.
    #[test]
    fn our_rows_are_read_by_language() {
        assert_eq!(lookup("fr", "Ask Nora…"), Some("Demander à Nora…"));
        assert_eq!(lookup("fr", "A string nobody wrote"), None);
        assert_eq!(lookup("en", "Ask Nora…"), None);
        assert_eq!(lookup("x-unknown", "Ask Nora…"), None);
        assert_ne!(lookup("de", "Ask Nora…"), lookup("fr", "Ask Nora…"));
        assert!(text("fr").is_some_and(|t| t.contains("\tAsk Nora…\tDemander à Nora…\n")));
        assert!(text("en").is_none());
    }

    /// Through `tr`: our row shows when the catalogs are embedded and have none, English never changes,
    /// and a test fixture's language owns its rows.
    #[test]
    fn tr_falls_back_to_our_rows() {
        if super::super::catalogs_embedded() {
            let fr = Lang::from_code("fr").expect("French is offered");
            assert_eq!(super::super::tr(fr, "Ask Nora…"), "Demander à Nora…");
            assert!(super::super::has(fr, "Ask Nora…"));
        }
        assert_eq!(super::super::tr(Lang::EN, "Ask Nora…"), "Ask Nora…");
        let fixture = super::super::fixture_lang("x-fixture", "\tHello\tBonjour\n");
        assert_eq!(super::super::tr(fixture, "Ask Nora…"), "Ask Nora…", "a fixture's language reads none of our rows");
        assert_eq!(super::super::tr(fixture, "Hello"), "Bonjour");
    }
}
