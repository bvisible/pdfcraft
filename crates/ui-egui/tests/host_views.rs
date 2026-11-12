// //// Neoffice — added file (no upstream equivalent): what a page's document might bring into a tab or a pane of its own.
//! A page that hosts the editor has one document, in one tab. Two things of upstream's (0.20 and 0.21) put a view of
//! something in a place of its own without a command that names it: a PDF portfolio opens the document its `/D` names in a tab
//! of its own, and split view shows the document twice, in two panes. The first is refused for a page that opens the
//! documents (a document the page never asked for, which it can't close); the second stays (it is the same document, no other
//! document and no other window), and leaves the session with that one document, which the page's guards still keep open.

use egui_kittest::Harness;
use pdfcraft_ui_egui::PdfCraftApp;

/// A one-page PDF.
const PAGE: &[u8] = b"%PDF-1.7
1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj
2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj
3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj
trailer << /Root 1 0 R >>
%%EOF";

/// A one-page portfolio of `files` (name, bytes), with `catalog` (its `/Collection`) in its catalog.
fn portfolio(catalog: &str, files: &[(&str, &[u8])]) -> Vec<u8> {
    let names: String = files.iter().enumerate().map(|(i, (name, _))| format!("({name}) {} 0 R ", 5 + 2 * i)).collect();
    let mut objs: Vec<Vec<u8>> = vec![
        format!("<< /Type /Catalog /Pages 2 0 R {catalog} /Names << /EmbeddedFiles << /Names [{names}] >> >> >>").into_bytes(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << >> >>".to_vec(),
    ];
    for (i, (name, data)) in files.iter().enumerate() {
        objs.push([format!("<< /Type /EmbeddedFile /Length {} >>\nstream\n", data.len()).as_bytes(), data, b"\nendstream"].concat());
        objs.push(format!("<< /Type /Filespec /F ({name}) /UF ({name}) /EF << /F {} 0 R >> >>", 4 + 2 * i).into_bytes());
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(o);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

fn opened(pdf: Vec<u8>, hosted: bool) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.host_owns_documents = hosted;
        app.open_bytes("page.pdf", None, pdf.clone()).unwrap();
        app
    });
    h.run_steps(6);
    h
}

fn names(h: &Harness<'static, PdfCraftApp>) -> Vec<String> {
    let s = h.state();
    s.views.iter().filter_map(|v| s.session.get(v.id)).map(|d| d.name.clone()).collect()
}

/// The documents the tabs show, each once.
fn documents(h: &Harness<'static, PdfCraftApp>) -> usize {
    let mut ids: Vec<_> = h.state().views.iter().map(|v| v.id).collect();
    ids.dedup();
    ids.sort_by_key(|id| format!("{id:?}"));
    ids.dedup();
    ids.len()
}

#[test]
fn a_portfolio_opens_its_first_document_in_a_tab_of_its_own_only_where_no_page_hosts_the_editor() {
    let pdf = portfolio("/Collection << /D (form.pdf) /View /T >>", &[("intro.pdf", PAGE), ("form.pdf", PAGE)]);
    let shown = opened(pdf.clone(), false);
    assert_eq!(names(&shown), ["page.pdf", "form.pdf"], "elsewhere the document `/D` names is presented in a tab of its own");

    let hosted = opened(pdf, true);
    assert_eq!(names(&hosted), ["page.pdf"], "the page's document is the only one: nothing opened by itself");
    assert_eq!(documents(&hosted), 1);
}

#[test]
fn split_view_shows_the_page_s_document_twice_and_never_another_one() {
    let mut h = opened(PAGE.to_vec(), true);
    assert_eq!(h.state().views.len(), 1);
    assert!(h.state_mut().execute("view.split"), "split view is not one of the commands a hosting page hides");
    h.run_steps(3);
    assert_eq!(h.state().views.len(), 2, "two views");
    assert_eq!(documents(&h), 1, "…of one document: no other document, no other window");
    assert_eq!(names(&h), ["page.pdf", "page.pdf"]);

    // The second view can't be closed the way another tab can, nor can the document by the commands that close it.
    h.state_mut().request_close_tab(1);
    h.state_mut().request_close_tab(0);
    assert!(!h.state_mut().execute("file.close"), "file.close ran");
    assert!(!h.state_mut().execute("file.close_all"), "file.close_all ran");
    h.run_steps(2);
    assert_eq!(h.state().views.len(), 2, "no view was closed");

    // Split view ends by its own command, and the page's document stays, in one tab.
    assert!(h.state_mut().execute("view.split"));
    h.run_steps(3);
    assert_eq!(h.state().views.len(), 1, "the second view is gone");
    assert_eq!(documents(&h), 1, "the page's document is still open");
}
