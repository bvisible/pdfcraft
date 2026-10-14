// //// Neoffice — added file (no upstream equivalent): tests of the embedding page's answers to a save (`pdfcraft:saved`).

//! A page that frames PdfCraft (`?host=parent`) is handed every saved file, and the browser build
//! marks the document saved at once. The page's `pdfcraft:saved` answer puts it back to unsaved
//! when it could not store the file. The browser branch of `save_then` only exists on wasm, so
//! these tests make its two session calls themselves (`hand_over`), then play the page's answers
//! through the queue the web shell fills (`RefusedSaves::answer`).

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use egui_kittest::Harness;
use pdfcraft_engine::Edit;
use pdfcraft_ui_egui::PdfCraftApp;

/// An `n`-page document with a proper xref table.
fn fixture(n: usize) -> Vec<u8> {
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")));
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into());
    for i in 0..n {
        objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i));
        let body = format!("BT /F1 24 Tf 20 150 Td (Page {}) Tj ET", i + 1);
        objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()));
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

/// A shell with one two-page document open per name; the first one is active.
fn harness(names: &[&str]) -> Harness<'static, PdfCraftApp> {
    let names: Vec<String> = names.iter().map(|n| (*n).to_owned()).collect();
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        for name in &names {
            app.open_bytes(name, None, fixture(2)).unwrap();
        }
        app.active = Some(0);
        app
    });
    h.run_steps(3);
    h
}

fn id_of(app: &PdfCraftApp, name: &str) -> pdfcraft_engine::DocId {
    app.session.docs().iter().find(|d| d.name == name).expect("the document is open").id
}

fn dirty(h: &Harness<'static, PdfCraftApp>, name: &str) -> bool {
    h.state().session.get(id_of(h.state(), name)).expect("the document is open").dirty
}

fn toast(h: &Harness<'static, PdfCraftApp>) -> String {
    h.state().toast.as_ref().map(|(m, _)| m.clone()).unwrap_or_default()
}

/// A change to `name`'s page `page` that the user would have to save.
fn edit(h: &mut Harness<'static, PdfCraftApp>, name: &str, page: usize) {
    let app = h.state_mut();
    app.active = app.views.iter().position(|v| v.id == id_of(app, name));
    assert!(app.apply_edit(Edit::RotatePages { pages: vec![page], degrees: 90 }), "the edit applies");
    h.run_steps(2);
}

/// What the browser's Save does once `host_save` has returned `Ok`: the bytes are written and the
/// document counts as saved (`save_then`, the wasm branch). Returns the bytes the page was given.
fn hand_over(h: &mut Harness<'static, PdfCraftApp>, name: &str) -> Arc<Vec<u8>> {
    let app = h.state_mut();
    let id = id_of(app, name);
    let bytes = app.session.save_bytes(id).unwrap();
    app.session.mark_saved(id, bytes.clone(), None).unwrap();
    h.run_steps(2);
    bytes
}

fn refuse(h: &Harness<'static, PdfCraftApp>, name: Option<&str>, error: Option<&str>) {
    assert!(h.state().refused_saves.answer(name.map(str::to_owned), Some(false), error.map(str::to_owned)), "a refusal is queued");
}

#[test]
fn a_refused_save_puts_the_document_back_to_unsaved_and_says_why() {
    let reports = Arc::new(Mutex::new(Vec::new()));
    let sink = reports.clone();
    let mut h = harness(&["report.pdf"]);
    h.state_mut().host_dirty = Some(Box::new(move |dirty| sink.lock().unwrap().push(dirty)));
    edit(&mut h, "report.pdf", 0);
    let id = id_of(h.state(), "report.pdf");
    let undo = h.state().session.get(id).unwrap().can_undo().map(str::to_owned);
    assert!(undo.is_some(), "the edit can be undone");
    assert!(h.state().unsaved_flag.load(Ordering::Relaxed));
    assert_eq!(reports.lock().unwrap().last(), Some(&true));

    // Handed to the page: saved, and it stays so until the page says otherwise (upstream's behaviour).
    let handed = hand_over(&mut h, "report.pdf");
    assert!(!dirty(&h, "report.pdf"));
    assert!(!h.state().unsaved_flag.load(Ordering::Relaxed));
    assert_eq!(reports.lock().unwrap().last(), Some(&false));

    // The page could not store it.
    refuse(&h, Some("report.pdf"), Some("Permission denied"));
    h.run_steps(2);
    assert!(dirty(&h, "report.pdf"), "the document has unsaved changes again");
    assert_eq!(h.state().first_dirty(), Some(0));
    assert!(h.state().unsaved_flag.load(Ordering::Relaxed), "the browser's reload warning follows");
    assert_eq!(reports.lock().unwrap().last(), Some(&true), "the page is told through pdfcraft:dirty");
    assert_eq!(toast(&h), "Couldn't save report.pdf: Permission denied");
    assert!(h.state().refused_saves.is_empty(), "an answer is read once");

    // Nothing else moved: the edit is still undoable, and saving again sends the same file.
    let doc = h.state().session.get(id).unwrap();
    assert_eq!(doc.can_undo().map(str::to_owned), undo);
    assert_eq!(h.state().session.save_bytes(id).unwrap(), handed);
    h.state_mut().session.undo(id).unwrap();
    assert!(dirty(&h, "report.pdf"), "undo leaves it unsaved");
}

#[test]
fn an_answer_that_is_not_a_refusal_changes_nothing() {
    let mut h = harness(&["report.pdf"]);
    edit(&mut h, "report.pdf", 0);
    hand_over(&mut h, "report.pdf");
    let refused = &h.state().refused_saves;
    assert!(!refused.answer(Some("report.pdf".into()), Some(true), None), "stored");
    assert!(!refused.answer(Some("report.pdf".into()), Some(true), Some("ignored".into())), "stored, whatever else it says");
    assert!(!refused.answer(Some("report.pdf".into()), None, Some("no ok".into())), "no verdict is not a refusal");
    assert!(refused.is_empty());
    h.run_steps(3);
    assert!(!dirty(&h, "report.pdf"));
    assert!(!h.state().unsaved_flag.load(Ordering::Relaxed));
    assert!(!toast(&h).contains("Couldn't save"), "the user is told nothing: {:?}", toast(&h));
}

#[test]
fn the_answer_is_for_the_document_that_carries_the_name() {
    let mut h = harness(&["a.pdf", "b.pdf"]);
    for name in ["a.pdf", "b.pdf"] {
        edit(&mut h, name, 0);
        hand_over(&mut h, name);
    }
    h.state_mut().active = Some(0);
    refuse(&h, Some("b.pdf"), Some("Locked by someone else"));
    h.run_steps(2);
    assert!(dirty(&h, "b.pdf") && !dirty(&h, "a.pdf"), "only the document with that name");
    assert_eq!(toast(&h), "Couldn't save b.pdf: Locked by someone else");
}

#[test]
fn two_documents_with_the_same_name_are_both_asked_to_be_saved_again() {
    // The name cannot tell them apart, so neither is trusted to be saved.
    let mut h = harness(&["same.pdf", "same.pdf"]);
    for index in 0..2 {
        h.state_mut().active = Some(index);
        assert!(h.state_mut().apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        let id = h.state().views[index].id;
        let bytes = h.state().session.save_bytes(id).unwrap();
        h.state_mut().session.mark_saved(id, bytes, None).unwrap();
    }
    h.run_steps(2);
    assert!(h.state().first_dirty().is_none());
    refuse(&h, Some("same.pdf"), Some("Disk full"));
    h.run_steps(2);
    assert_eq!(h.state().session.docs().iter().filter(|doc| doc.dirty).count(), 2);
}

#[test]
fn an_answer_that_no_document_carries_is_taken_for_the_active_one() {
    let mut h = harness(&["a.pdf", "b.pdf"]);
    for name in ["a.pdf", "b.pdf"] {
        edit(&mut h, name, 0);
        hand_over(&mut h, name);
    }
    // A document closed right after its save, or a page that changed the name: the safe side of a
    // doubt is the active document, and the user is told which file the page meant.
    h.state_mut().active = Some(1);
    refuse(&h, Some("gone.pdf"), Some("Disk full"));
    h.run_steps(2);
    assert!(dirty(&h, "b.pdf") && !dirty(&h, "a.pdf"));
    assert_eq!(toast(&h), "Couldn't save gone.pdf: Disk full");

    // No name at all: the same, and the notice names the active document.
    hand_over(&mut h, "b.pdf");
    h.state_mut().active = Some(0);
    refuse(&h, None, Some("Offline"));
    h.run_steps(2);
    assert!(dirty(&h, "a.pdf") && !dirty(&h, "b.pdf"));
    assert_eq!(toast(&h), "Couldn't save a.pdf: Offline");

    // Home showing, no document active: nothing to mark, the notice still comes.
    hand_over(&mut h, "a.pdf");
    h.state_mut().active = None;
    refuse(&h, Some("gone.pdf"), Some("Disk full"));
    h.run_steps(2);
    assert!(!dirty(&h, "a.pdf") && !dirty(&h, "b.pdf"));
    assert_eq!(toast(&h), "Couldn't save gone.pdf: Disk full");
}

#[test]
fn the_same_name_saved_twice_before_the_answer_is_answered_for_the_last_save() {
    let mut h = harness(&["report.pdf"]);
    edit(&mut h, "report.pdf", 0);
    let first = hand_over(&mut h, "report.pdf");
    edit(&mut h, "report.pdf", 1);
    let second = hand_over(&mut h, "report.pdf");
    assert_ne!(first, second);

    // One answer comes back, for the name: it counts for the last save.
    refuse(&h, Some("report.pdf"), Some("Disk full"));
    h.run_steps(2);
    assert!(dirty(&h, "report.pdf"));
    let id = id_of(h.state(), "report.pdf");
    assert_eq!(h.state().session.save_bytes(id).unwrap(), second, "saving again sends both edits");

    // A second refusal of the same name (the first save's answer, late) is shown too, and breaks nothing.
    refuse(&h, Some("report.pdf"), Some("Disk full"));
    h.run_steps(2);
    assert!(dirty(&h, "report.pdf"));
    assert_eq!(toast(&h), "Couldn't save report.pdf: Disk full");
}

#[test]
fn a_missing_or_huge_reason_does_not_break_the_notice() {
    let mut h = harness(&["report.pdf"]);
    edit(&mut h, "report.pdf", 0);
    hand_over(&mut h, "report.pdf");

    // Worded as the other Craft editors' bridges word it.
    refuse(&h, Some("report.pdf"), None);
    h.run_steps(2);
    assert_eq!(toast(&h), "Couldn't save report.pdf: the page did not say why");
    assert!(dirty(&h, "report.pdf"));

    refuse(&h, Some("report.pdf"), Some("  \n "));
    h.run_steps(2);
    assert_eq!(toast(&h), "Couldn't save report.pdf: the page did not say why", "a blank reason is no reason");

    // One line, at most 300 characters, the ellipsis included.
    refuse(&h, Some("report.pdf"), Some(&"x".repeat(1000)));
    h.run_steps(2);
    assert_eq!(toast(&h), format!("Couldn't save report.pdf: {}…", "x".repeat(299)), "the page cannot flood the notice");

    refuse(&h, Some("report.pdf"), Some("  Quota\nexceeded \n\n for  this folder "));
    h.run_steps(2);
    assert_eq!(toast(&h), "Couldn't save report.pdf: Quota exceeded for this folder");
}
