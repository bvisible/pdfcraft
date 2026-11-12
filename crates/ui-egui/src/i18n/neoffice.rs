// //// Neoffice — added file (no upstream equivalent)
//! The strings Neoffice added to PdfCraft (the Nora assistant pane, its switch in Preferences and its
//! command), translated here, outside the craft-i18n catalogs. Upstream moved its catalogs out of the
//! repository in 0.19 (storytold/craft-i18n, embedded from `CRAFT_I18N_DIR`), so a row of ours has no
//! place there; the rows we had written for the 34 languages are kept in `neoffice/<code>.tsv`, in the
//! catalogs' own format (context, English source and translation, tab separated; `#` comments).
//!
//! [`super::tr`] asks here only when the language's catalogs hold no entry for a string, so a catalog
//! that gains the same English string wins and an upstream merge never touches these. Only plain rows
//! (an empty context) are read: nothing of ours needs a context, an `@id` or a plural form.
//!
//! A language without a file here (English) shows the English string. The font tests read these rows
//! through [`super::Lang::catalog_texts`], as they read the catalogs.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::{LANGUAGES, Lang};

/// The rows of each language, by its code.
const SOURCES: &[(&str, &str)] = &[
    ("ar", include_str!("neoffice/ar.tsv")),
    ("bg", include_str!("neoffice/bg.tsv")),
    ("ca", include_str!("neoffice/ca.tsv")),
    ("ckb", include_str!("neoffice/ckb.tsv")),
    ("cs", include_str!("neoffice/cs.tsv")),
    ("de", include_str!("neoffice/de.tsv")),
    ("el", include_str!("neoffice/el.tsv")),
    ("es", include_str!("neoffice/es.tsv")),
    ("fa", include_str!("neoffice/fa.tsv")),
    ("fi", include_str!("neoffice/fi.tsv")),
    ("fr", include_str!("neoffice/fr.tsv")),
    ("hi", include_str!("neoffice/hi.tsv")),
    ("hu", include_str!("neoffice/hu.tsv")),
    ("id", include_str!("neoffice/id.tsv")),
    ("it", include_str!("neoffice/it.tsv")),
    ("ja", include_str!("neoffice/ja.tsv")),
    ("kk", include_str!("neoffice/kk.tsv")),
    ("ko", include_str!("neoffice/ko.tsv")),
    ("lo", include_str!("neoffice/lo.tsv")),
    ("lt", include_str!("neoffice/lt.tsv")),
    ("nb", include_str!("neoffice/nb.tsv")),
    ("pl", include_str!("neoffice/pl.tsv")),
    ("pt-br", include_str!("neoffice/pt-br.tsv")),
    ("ro", include_str!("neoffice/ro.tsv")),
    ("ru", include_str!("neoffice/ru.tsv")),
    ("sl", include_str!("neoffice/sl.tsv")),
    ("sq", include_str!("neoffice/sq.tsv")),
    ("te", include_str!("neoffice/te.tsv")),
    ("th", include_str!("neoffice/th.tsv")),
    ("tr", include_str!("neoffice/tr.tsv")),
    ("uk", include_str!("neoffice/uk.tsv")),
    ("vi", include_str!("neoffice/vi.tsv")),
    ("zh-hans", include_str!("neoffice/zh-hans.tsv")),
    ("zh-hant", include_str!("neoffice/zh-hant.tsv")),
];

/// Per language of [`LANGUAGES`], its rows by English source, read on first use.
static ROWS: [OnceLock<HashMap<&'static str, &'static str>>; super::LANGUAGE_COUNT] = [const { OnceLock::new() }; super::LANGUAGE_COUNT];

/// The text of the rows file of `code` (any case), for the checks that the fonts draw every translation.
pub(super) fn text(code: &str) -> Option<&'static str> {
    SOURCES.iter().find(|(c, _)| c.eq_ignore_ascii_case(code)).map(|(_, text)| *text)
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
    let slot = LANGUAGES.iter().position(|l| l.code == code).and_then(|i| ROWS.get(i))?;
    let rows = slot.get_or_init(|| text(code).map(parse).unwrap_or_default());
    rows.get(source).copied()
}

/// The plain rows of `text`: comments, blank lines, rows with a context and malformed rows are left
/// out; a source that appears twice keeps its first row. No escape is read: a row holds no tab or
/// line break (the test `no_row_needs_an_escape` keeps it so).
fn parse(text: &'static str) -> HashMap<&'static str, &'static str> {
    let mut rows = HashMap::new();
    for line in text.lines().map(|l| l.strip_suffix('\r').unwrap_or(l)) {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let mut columns = line.split('\t');
        if let (Some(""), Some(source), Some(translation), None) = (columns.next(), columns.next(), columns.next(), columns.next())
            && !translation.is_empty()
        {
            rows.entry(source).or_insert(translation);
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every language we offer, English apart, has a file, and every file is a language we offer.
    #[test]
    fn every_language_has_its_rows_and_no_file_is_stray() {
        for info in &LANGUAGES {
            assert_eq!(text(info.code).is_some(), info.code != "en", "{}", info.code);
        }
        for (code, _) in SOURCES {
            assert!(LANGUAGES.iter().any(|l| l.code == *code), "{code} is not an offered language");
        }
    }

    /// Reading a row literally is right only while none holds an escape.
    #[test]
    fn no_row_needs_an_escape() {
        for (code, text) in SOURCES {
            for line in text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
                assert!(!line.contains('\\'), "{code}: an escape in {line:?}");
                assert_eq!(line.split('\t').count(), 3, "{code}: not a plain row: {line:?}");
                assert!(line.starts_with('\t'), "{code}: a row with a context: {line:?}");
            }
        }
    }

    /// Our 34 languages all say the same things: a row of the French file has its counterpart in each other file,
    /// with the same `{placeholders}`.
    #[test]
    fn every_language_translates_the_same_sources_and_keeps_their_placeholders() {
        let fr = parse(text("fr").expect("French rows"));
        assert!(fr.len() > 70, "the French rows: {}", fr.len());
        fn holes(s: &str) -> Vec<&str> {
            let mut found: Vec<&str> = s.split('{').skip(1).filter_map(|p| p.split_once('}').map(|(name, _)| name)).collect();
            found.sort_unstable();
            found
        }
        for (code, text) in SOURCES {
            let rows = parse(text);
            for (source, translation) in &rows {
                assert_eq!(holes(source), holes(translation), "{code}: placeholders of {source:?}");
            }
            let missing: Vec<&&str> = rows.keys().filter(|s| !fr.contains_key(**s)).collect();
            assert!(missing.is_empty(), "{code} translates sources French lacks: {missing:?}");
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
