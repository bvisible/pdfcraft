// //// Neoffice — added file (no upstream equivalent): the Nora assistant pane, built on craft-nora and craft-nora-ui.

//! The Nora pane: Neoffice's assistant, a chat at the right of the window. It is the pane NeoWrite (WordCraft) has, so
//! that the same Nora works in every editor.
//!
//! Nora (the agent, her memory and her tools) lives on the Neoffice server. This pane is her chat window: it keeps a
//! [`NoraSession`] (the conversation, from `craft-nora`) and draws it with [`NoraPanel`] (from `craft-nora-ui`). It is
//! off until Preferences ▸ Assistant or the `ui.nora` command turns it on; a host that offers Nora (the browser build in
//! Neoffice's Drive, Neodesk) opens it.
//!
//! The pane never opens a connection itself. The host makes the HTTP calls with its own session and credentials
//! ([`PdfCraftApp::nora_http`]) and delivers the answers ([`PdfCraftApp::nora_inbox`]). Without a connection the pane
//! says that Nora isn't available and offers no text box.
//!
//! What Nora asks for in the document (the `doc_*` actions) is played in the document of the active tab:
//! - **Observe**: the text of its pages, the visible ones first, then the others in order, cut at a length that fits
//!   her; the page count, the current page and the selected text.
//! - **Execute** and **Batch**: PdfCraft's commands, played by the headless tools of `pdfcraft-automation`, the surface
//!   the MCP server and `pdfcraft-cli run` use: a tool by its name (`page_rotate`, `comment_add`), or a registry command
//!   by its id (`page.rotate`), played by the one tool that automates it. Never the person's: not the interface, their
//!   undo history, their clipboard, other documents and files, printing, signing, links or scripts ([`REFUSED`]), and
//!   no parameter that names a file of the computer or a password, at any depth and in any case.
//! - **SelectText**: the n-th occurrence of the text, pages in order, selected as the person would select it.
//! - **TypeText**: into the form field being filled, else into a text box comment on the current page, in free space
//!   (never over its text, comments or fields), else at the top of the page.
//! - **RenderPage**: a page as a picture (`render_preview`).
//! - **Save**: the app's own Save (for a document without a file, its Save As: the hosting page, a dialog or a download).
//! - **Undo** (the pane's button): takes back Nora's last change whole, and only while the document is as she left it.
//!
//! A request that fails leaves the document as it was, its undo and redo history and its unsaved flag included. Nora's
//! actions wait while the person types in the document, drags or resizes something, or has a dialog open.

use std::sync::{Arc, LazyLock};
use std::time::Duration;

use craft_nora::{
    AppKind, CommandCall, CommandPolicy, Decision, DocActionKind, DocumentCall, DocumentInfo, HttpRequest, HttpResponse, Input, NoraError,
    NoraSession, Output, PageContext, Selection, SessionConfig,
};
use craft_nora_ui::{NoraLabels, NoraPanel, NoraStyle, PanelAction, drive};
use egui::{Stroke, Ui};
use pdfcraft_automation::{Automation, Content, ToolDef};
use pdfcraft_engine::{DocId, Document, Edit, FieldValue, FormFieldKind, field_flags};
use pdfcraft_render::{PageRenderer, PageText, RenderConfig, RenderRequest, RequestKind};
use serde_json::{Map, Value, json};

use crate::PdfCraftApp;
use crate::i18n::fmt;
use crate::theme::{self, Tokens};

/// The most characters of the selection told to Nora with a message; she reads the document herself when she needs more.
const MAX_SELECTION_CHARS: usize = 2_000;

/// The most characters of the document told to Nora when she reads it (as in WordCraft).
const MAX_OBSERVED_CHARS: usize = 50_000;

/// The most pages one reading extracts: the rest of a long document is hers to read with `text_extract`.
const MAX_OBSERVED_PAGES: usize = 50;

/// The most pages a search for a text to select goes through.
const MAX_SEARCHED_PAGES: usize = 500;

/// The most commands of one batch: well under the 100 steps the engine's undo history keeps, so that the pane's Undo
/// can always take a batch back whole.
const MAX_BATCH: usize = 50;

/// How deep parameters are looked into (the JSON they come from is never deeper than 128).
const MAX_DEPTH: usize = 64;

/// Parameter names that reach a file of the computer, at any depth and in any case: a command given one is refused.
/// (`dir`, a direction elsewhere, is no name of PdfCraft's tools; `file_page` is a page number.)
const FILE_KEYS: &[&str] =
    &["paths", "file", "files", "filename", "file_name", "folder", "folders", "directory", "directories", "out", "out_dir", "output"];

/// Parameter names that name a file when their value holds text: `content_update`'s `image` is the file of a new picture,
/// `image_edit`'s the number of a picture on the page; `doc_open`'s `path` is a file, the bookmark tools' the numbers
/// that lead to a bookmark.
const FILE_KEYS_IF_TEXT: &[&str] = &["image", "path"];

/// Values of a parameter of a tool that make the call the person's: Fill & Sign's typed signature and initials (theirs
/// to add, as `sign.` below), where its text, marks and dates are Nora's to place.
const REFUSED_VALUES: &[(&str, &str, &[&str])] = &[("fill_sign_add", "type", &["signature", "initials"])];

/// The tools that give the text of a document's pages, refused in one whose security forbids copying its text (as
/// Observe gives none of it, and SelectText and RenderPage are refused): its lines, its paragraphs and objects, what OCR
/// or the accessibility tags read in it, what a redaction covers, and a search's matches and their places, which would
/// spell it out.
const COPYING_TOOLS: &[&str] = &[
    "text_find",
    "text_lines",
    "text_extract",
    "text_paragraphs",
    "object_list",
    "ocr_recognize",
    "accessibility_autotag_preview",
    "doc_audit_redactions",
];

/// What Nora may not play, besides the default policy's files (`file.`) and the application (`app.`): registry ids or
/// their prefixes (`view.`), and the names (or prefixes) of the tools that would play them (`view_reading_colors`). A
/// registry command that a tool plays is checked under both names.
const REFUSED: &[&str] = &[
    // The interface, which is the person's: views, zoom, page display, full screen, the theme, panels, this pane, the
    // help links, and the settings that change how documents show (hidden comments, reading colours, the date format).
    "ui.",
    "view.",
    "help.",
    "comment.list",
    "comment.hide_all",
    "comments_hide",
    "a11y.reading_options",
    "view_",
    "fill_sign_date_format",
    // The person's own undo history: the pane's Undo would then take back their work as Nora's.
    "edit.undo",
    "edit.redo",
    "edit_undo",
    "edit_redo",
    // Their clipboard.
    "edit.snapshot",
    "page.copy",
    "page.cut",
    "page.paste",
    // Other documents and windows, and the computer's files: opening, creating, combining, splitting, extracting,
    // comparing, closing, saving (Nora's Save is an action of its own, always asked), exporting and printing.
    "create.",
    "page.combine",
    "page.extract",
    "page.split",
    "doc.compare",
    "print.",
    "export.",
    "optimize.reduce",
    "comment.import",
    "comment.export",
    "form.import_data",
    "form.export_data",
    "form.merge_data",
    "ocr.recognize_batch",
    "measure.export",
    "a11y.report",
    "doc_open",
    "doc_list",
    "doc_close",
    "doc_save",
    "doc_create",
    "doc_combine",
    "doc_split",
    "doc_compare",
    "doc_export_",
    "doc_import_data",
    "doc_reduce",
    "doc_print",
    "printer",
    "page_extract",
    "page_insert_file",
    "page_replace",
    "comments_summarize",
    "accessibility_report",
    "accessibility_review_",
    "measure_export",
    "image_save",
    "form_merge_data",
    "ocr_recognize_files",
    "text_font_files",
    "text_font_import",
    // Links, scripts, and stored automation that runs other commands.
    "link_open",
    "js_",
    "tools.",
    "form_set_script",
    "form_set_actions",
    "actions.",
    "action_",
    "command_run",
    "command_batch",
    // Signing: the person's digital IDs, their signature and initials, the certificates they trust.
    "sign.",
    "sign_id_create",
    "sign_windows_ids",
    "sign_keychain_ids",
    "sign_document",
    "sign_trust",
];

/// The tool table of `pdfcraft-automation`.
static TOOLS: LazyLock<Vec<ToolDef>> = LazyLock::new(pdfcraft_automation::tools);

/// What Nora may play in a document: see [`REFUSED`].
static POLICY: LazyLock<CommandPolicy> = LazyLock::new(policy);

fn policy() -> CommandPolicy {
    REFUSED.iter().fold(CommandPolicy::default(), |policy, prefix| policy.deny_prefix(*prefix))
}

/// The size of the text in a text box of Nora's, in points.
const TEXT_SIZE: f32 = 12.0;

/// The room kept free along the edges of a page for Nora's text box, in points (half an inch).
const MARGIN: f32 = 36.0;

/// The room kept between Nora's text box and what is on the page, and the step of the search for free space, in points.
const GAP: f32 = 6.0;

/// The most places the search for free space tries along each side of a page (a huge page takes longer steps), and the
/// most rectangles it checks them against (past them, the box goes to its corner): a page can't hold the window.
const MAX_TRIES: usize = 200;
const MAX_TAKEN: usize = 4_000;

/// What the pane keeps between frames: the conversation, the panel that draws it, what Nora changed, the actions that
/// wait for the person, and the tools her commands are played with.
pub(crate) struct NoraState {
    session: NoraSession,
    panel: NoraPanel,
    /// The changes of Nora's that the pane's Undo can take back, the newest last.
    changes: Vec<Change>,
    /// Nora's actions that wait for the person (see [`person_busy`]), in the order she asked for them.
    waiting: Vec<DocumentCall>,
    /// The tools Nora's commands are played with: the window's session is swapped into it for each call, and it is made
    /// anew after each action.
    automation: Automation,
}

/// One change Nora made: in which document, how many undo steps it took, and the document's generation (the engine's
/// counter of changes to its working file) before it and as she left it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Change {
    doc: DocId,
    steps: usize,
    before: u64,
    after: u64,
}

impl Default for NoraState {
    fn default() -> Self {
        let context = PageContext::editor(AppKind::Pdf, DocumentInfo::named(""));
        let config = SessionConfig { policy: POLICY.clone(), ..SessionConfig::default() };
        NoraState {
            session: NoraSession::new(config, context),
            panel: NoraPanel::new().with_error_words(error_words),
            changes: Vec::new(),
            waiting: Vec::new(),
            automation: Automation::new(),
        }
    }
}

/// The pane at the right of the window: a title row like the other panels' ("Nora" and a close button), then the
/// conversation.
pub(crate) fn pane(app: &mut PdfCraftApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::right("nora_pane")
        .resizable(true)
        .default_size(330.0)
        .size_range(260.0..=560.0)
        .frame(
            egui::Frame::NONE
                .fill(t.panel)
                .inner_margin(egui::Margin { left: 14, right: 12, top: 12, bottom: 8 })
                .stroke(Stroke::new(1.0, t.divider)),
        )
        .show(ui, |ui| {
            if header(ui, &t) {
                app.nora_pane = false;
                return;
            }
            show(app, ui, &t);
        });
}

/// The title row: the pane's name, and a close button. True when the button was clicked.
fn header(ui: &mut Ui, t: &Tokens) -> bool {
    let mut close = false;
    ui.horizontal(|ui| {
        let title_width = (ui.available_width() - 26.0 - ui.spacing().item_spacing.x).max(1.0);
        // Nora is a name, the same in every language.
        ui.add_sized([title_width, 26.0], egui::Label::new(egui::RichText::new("Nora").font(theme::semibold(15.5)).color(t.text)).truncate());
        close = crate::icons::button(ui, "x", 26.0, false, tl!("Close panel")).clicked();
    });
    ui.add_space(2.0);
    close
}

/// Draws the conversation (inside the pane's frame) and moves it along.
fn show(app: &mut PdfCraftApp, ui: &mut Ui, t: &Tokens) {
    let ctx = ui.ctx().clone();
    // The panel's text box (craft-nora-ui gives it this id): typing there is talking to Nora, not typing in the document.
    let nora_input = ui.id().with("nora").with("input");
    let busy = person_busy(app, &ctx, nora_input);
    let mut outputs = incoming(app, busy);
    outputs.extend(drive(&ctx, &mut app.nora.session));
    app.nora.panel.labels = labels();
    app.nora.panel.style = style(t);
    app.nora.panel.offline = app.nora_http.is_none().then(|| tl!("Nora isn't available in this version yet").to_string());
    if !app.nora.waiting.is_empty() {
        ui.add(
            egui::Label::new(egui::RichText::new(tl!("Nora is waiting for you to finish what you are doing in the document.")).color(t.text_muted))
                .wrap(),
        );
        ui.add_space(4.0);
    }
    for action in app.nora.panel.show(ui, &app.nora.session) {
        match &action {
            // Where the person is, as of the message: told to Nora with it, never read on every frame.
            PanelAction::Send(_) => {
                let context = context_of(app);
                outputs.extend(app.nora.session.handle(Input::SetContext(context)));
            }
            // A new conversation can't undo the old one's changes, and what the old one asked for no longer runs.
            PanelAction::NewConversation => {
                app.nora.changes.clear();
                app.nora.waiting.clear();
            }
            _ => {}
        }
        outputs.extend(app.nora.session.handle(action.into_input()));
    }
    perform(app, outputs, &ctx, busy);
    if !app.nora.waiting.is_empty() {
        // Checks again soon whether the person is done.
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

/// What the conversation has for this frame: the actions that waited for the person, once they are done, then what
/// the host's answers ask for.
fn incoming(app: &mut PdfCraftApp, busy: bool) -> Vec<Output> {
    let mut outputs: Vec<Output> = if busy { Vec::new() } else { std::mem::take(&mut app.nora.waiting).into_iter().map(Output::Document).collect() };
    outputs.extend(deliver_answers(app));
    outputs
}

/// The answers the host delivered since the last frame, fed to the conversation.
fn deliver_answers(app: &mut PdfCraftApp) -> Vec<Output> {
    let Some(inbox) = app.nora_inbox.clone() else { return Vec::new() };
    let answers = std::mem::take(&mut *inbox.lock().unwrap_or_else(|e| e.into_inner()));
    answers.into_iter().flat_map(|answer| app.nora.session.handle(Input::Http(answer))).collect()
}

/// Whether the person is in the middle of something Nora's actions must not cut into: a button held (a drag, a
/// resize, a click), a dialog, a prompt or the command palette over the window, the keyboard in a text box that isn't
/// Nora's own, or typing, drawing or choosing in the document.
fn person_busy(app: &PdfCraftApp, ctx: &egui::Context, nora_input: egui::Id) -> bool {
    if ctx.dragged_id().is_some() || ctx.input(|i| i.pointer.any_down()) {
        return true;
    }
    if app.modal_open() || app.palette_open {
        return true;
    }
    if ctx.memory(|m| m.focused()).is_some_and(|id| id != nora_input && egui::TextEdit::load_state(ctx, id).is_some()) {
        return true;
    }
    working_in_document(app)
}

/// What the person has under way in the active document: an edit queued this frame, a choice list open, a comment's
/// note or a new comment with text not posted, a shape being drawn, a text box being added or edited, a Fill & Sign
/// text, a bookmark being renamed. (A form field's text editor counts while it has the keyboard; one that doesn't is
/// the field Nora types into.)
fn working_in_document(app: &PdfCraftApp) -> bool {
    if app.bookmark_rename.is_some() {
        return true;
    }
    let Some(view) = app.active.and_then(|i| app.views.get(i)) else { return false };
    let doc = app.session.get(view.id);
    let choosing = view
        .forms
        .focus
        .as_ref()
        .is_some_and(|focus| doc.and_then(|d| d.form.iter().find(|f| f.name == focus.name)).is_some_and(|f| f.kind != FormFieldKind::Text));
    let note = view.comments.editing.as_ref().is_some_and(|(page, index, text)| {
        let stored = doc.and_then(|d| d.info.annotations.iter().find(|a| a.page == *page && a.index == *index)).and_then(|a| a.contents.as_deref());
        stored.unwrap_or_default() != text.as_str()
    });
    view.pending_edit.is_some()
        || choosing
        || note
        || view.comments.composer.as_ref().is_some_and(|c| !c.text.trim().is_empty())
        || view.comments.gesture.is_some()
        || view.content.draft.is_some()
        || view.line_editor.is_some()
        || view.fill_text.is_some()
}

/// Does what the conversation asks: a call goes to the host, a document action is played (or waits for the person), a
/// change repaints. The results and the missing-connection answers come back as inputs and may ask for more, so this
/// goes round a few times at most.
fn perform(app: &mut PdfCraftApp, outputs: Vec<Output>, ctx: &egui::Context, busy: bool) {
    let mut queue = outputs;
    for _ in 0..8 {
        if queue.is_empty() {
            return;
        }
        let mut next = Vec::new();
        for output in std::mem::take(&mut queue) {
            match output {
                Output::Http(request) => next.extend(send(app, request)),
                // Behind the ones that already wait, too, so that they keep their order.
                Output::Document(call) if busy || !app.nora.waiting.is_empty() => app.nora.waiting.push(call),
                Output::Document(call) => {
                    let result = run_document(app, &call.kind);
                    next.extend(app.nora.session.handle(Input::DocumentResult { id: call.id, result }));
                    ctx.request_repaint();
                }
                Output::Changed => ctx.request_repaint(),
            }
        }
        queue = next;
    }
}

/// The active document and its generation: the state Nora's changes are compared with.
fn revision_of(app: &PdfCraftApp) -> Option<(DocId, u64)> {
    let (_, id) = app.active_ids()?;
    Some((id, app.session.get(id)?.edit_generation()))
}

/// Plays one action of Nora's in the active document.
///
/// One that can change the document is played against a checkpoint: if it fails, the document goes back to it, undo
/// and redo history and unsaved flag included; if it changed something, the change is remembered with the undo steps it
/// took, for the pane's Undo. One that changed nothing says so (`"changed": false`). An action that leaves the document
/// as Nora's last change left it (a reading, a save, a request put back) keeps that change undoable.
fn run_document(app: &mut PdfCraftApp, kind: &DocActionKind) -> Result<Value, String> {
    if *kind == DocActionKind::Undo {
        return undo(app);
    }
    let at_last = revision_of(app).is_some_and(|(id, generation)| app.nora.changes.last().is_some_and(|c| c.doc == id && c.after == generation));
    let changes = app.nora.changes.len();
    let result = if kind.modifies() { change(app, kind) } else { apply(app, kind) };
    // The tools' caches (renderers, page texts) hold the documents' bytes: they last one action, so that a document
    // the person closes is not kept in memory by them.
    app.nora.automation = Automation::new();
    if at_last
        && app.nora.changes.len() == changes
        && let Some((id, generation)) = revision_of(app)
        && let Some(last) = app.nora.changes.last_mut().filter(|c| c.doc == id)
    {
        last.after = generation;
    }
    result
}

/// An action that can change the active document (Execute, Batch, TypeText), played against a checkpoint.
fn change(app: &mut PdfCraftApp, kind: &DocActionKind) -> Result<Value, String> {
    let (_, id) = app.active_ids().ok_or_else(no_document)?;
    let checkpoint = app.session.checkpoint(id).ok_or_else(no_document)?;
    let generation = |app: &PdfCraftApp| app.session.get(id).map_or(0, Document::edit_generation);
    let before = generation(app);
    let result = apply(app, kind);
    let after = generation(app);
    match result {
        Ok(mut value) => {
            let steps = usize::try_from(after.saturating_sub(before)).unwrap_or(usize::MAX);
            if steps == 0 {
                unchanged(&mut value);
            } else {
                app.nora.changes.push(Change { doc: id, steps, before, after });
                refresh(app, id);
            }
            Ok(value)
        }
        Err(error) => {
            if after != before {
                if let Err(e) = app.session.restore(checkpoint) {
                    log::error!("the document could not be put back after Nora's failed request: {e}");
                }
                refresh(app, id);
            }
            Err(error)
        }
    }
}

/// Marks a result as one that changed nothing: Nora's Undo then has nothing of it to take back.
fn unchanged(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.insert("changed".into(), Value::Bool(false));
        }
        other => *other = json!({ "result": other.take(), "changed": false }),
    }
}

/// Whatever is drawn from document `id` is drawn again.
fn refresh(app: &mut PdfCraftApp, id: DocId) {
    if let (Some(doc), Some(view)) = (app.session.get(id), app.views.iter_mut().find(|v| v.id == id)) {
        view.document_changed(&doc.info);
    }
}

fn apply(app: &mut PdfCraftApp, kind: &DocActionKind) -> Result<Value, String> {
    match kind {
        DocActionKind::Observe => observe(app),
        DocActionKind::Execute { command, params } => execute(app, command, params),
        DocActionKind::Batch { commands } => batch(app, commands),
        DocActionKind::SelectText { text, occurrence } => select_text(app, text, *occurrence),
        DocActionKind::TypeText { text, paragraphs } => type_text(app, text, *paragraphs),
        DocActionKind::RenderPage { page, scale } => render_page(app, *page, *scale),
        DocActionKind::Save => save(app),
        DocActionKind::Undo => undo(app),
    }
}

fn no_document() -> String {
    tl!("There is no document open for Nora to work in.").to_string()
}

/// The tool a command of Nora's names: a tool by its own name (`page_rotate`, as the MCP server lists them), or the one
/// tool that automates a registry command (`page.rotate`). Refused when either name is the person's ([`REFUSED`]).
fn tool_for(command: &str) -> Result<&'static ToolDef, String> {
    let refused = |name: &str| POLICY.check(name) == Decision::Deny;
    let refusal = || fmt(tl!("`{command}` is refused: Nora may not run it."), &[("command", command)]);
    if refused(command) {
        return Err(refusal());
    }
    let tool = match TOOLS.iter().find(|t| t.name == command) {
        Some(tool) => tool,
        None if pdfcraft_engine::commands::command(command).is_some() => {
            let played: Vec<&'static ToolDef> = TOOLS.iter().filter(|t| t.command == Some(command)).collect();
            match played.as_slice() {
                [tool] => *tool,
                [] => {
                    return Err(fmt(
                        tl!("`{command}` can't be run by Nora in NeoPDF: it works only through the interface."),
                        &[("command", command)],
                    ));
                }
                several => {
                    let names: Vec<&str> = several.iter().map(|t| t.name).collect();
                    return Err(fmt(
                        tl!("`{command}` is played by several tools ({tools}): name the one to run."),
                        &[("command", command), ("tools", &names.join(", "))],
                    ));
                }
            }
        }
        None => return Err(fmt(tl!("`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run."), &[("command", command)])),
    };
    if refused(tool.name) {
        return Err(refusal());
    }
    Ok(tool)
}

/// One command of Nora's: its tool, run in the active document, unless a parameter would reach a file of the computer
/// or names a password.
fn execute(app: &mut PdfCraftApp, command: &str, params: &Value) -> Result<Value, String> {
    let tool = tool_for(command)?;
    if too_deep(params, 0) {
        return Err(fmt(tl!("`{command}` is refused: its parameters are nested too deeply."), &[("command", command)]));
    }
    if let Some(key) = file_key(params) {
        return Err(fmt(tl!("`{command}` is refused: `{key}` would reach a file of the computer."), &[("command", command), ("key", &key)]));
    }
    if let Some(key) = password_key(params) {
        return Err(fmt(tl!("`{command}` is refused: `{key}` is a password, which only you enter."), &[("command", command), ("key", &key)]));
    }
    if refused_value(tool.name, params).is_some() {
        return Err(fmt(tl!("`{command}` is refused: your signature and initials are yours to add."), &[("command", command)]));
    }
    if COPYING_TOOLS.contains(&tool.name) && forbids_copying(app) {
        return Err(no_copying(command));
    }
    let args = arguments(app, tool, command, params)?;
    call(app, tool.name, &args).map(output)
}

/// Whether `params` are nested deeper than any command's.
fn too_deep(params: &Value, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        return true;
    }
    match params {
        Value::Object(map) => map.values().any(|v| too_deep(v, depth + 1)),
        Value::Array(list) => list.iter().any(|v| too_deep(v, depth + 1)),
        _ => false,
    }
}

/// The first parameter, at any depth, whose name `matches` (lowercased) with its value.
fn find_key(params: &Value, matches: &dyn Fn(&str, &Value) -> bool) -> Option<String> {
    match params {
        Value::Object(map) => {
            map.iter().find_map(|(key, value)| if matches(&key.to_ascii_lowercase(), value) { Some(key.clone()) } else { find_key(value, matches) })
        }
        Value::Array(list) => list.iter().find_map(|value| find_key(value, matches)),
        _ => None,
    }
}

/// The first parameter name, at any depth and in any case, that would reach a file of the computer.
fn file_key(params: &Value) -> Option<String> {
    find_key(params, &|key, value| FILE_KEYS.contains(&key) || (FILE_KEYS_IF_TEXT.contains(&key) && holds_text(value, 0)))
}

/// Whether `value` is text or holds some, at any depth.
fn holds_text(value: &Value, depth: usize) -> bool {
    match value {
        Value::String(_) => true,
        _ if depth > MAX_DEPTH => true,
        Value::Array(list) => list.iter().any(|v| holds_text(v, depth + 1)),
        Value::Object(map) => map.values().any(|v| holds_text(v, depth + 1)),
        _ => false,
    }
}

/// Whether the active document's security forbids copying its text (and pictures of it): Nora gets none of it.
fn forbids_copying(app: &PdfCraftApp) -> bool {
    app.active_ids().and_then(|(_, id)| app.session.get(id)).is_some_and(|doc| !doc.allows_copying())
}

fn no_copying(command: &str) -> String {
    fmt(tl!("`{command}` is refused: this document's security forbids copying its text."), &[("command", command)])
}

/// The parameter of `tool` whose value makes the call the person's ([`REFUSED_VALUES`]).
fn refused_value(tool: &str, params: &Value) -> Option<&'static str> {
    REFUSED_VALUES.iter().find_map(|(name, key, values)| {
        let value = params.get(*key).and_then(Value::as_str).map(|v| v.trim().to_ascii_lowercase());
        (*name == tool && value.is_some_and(|v| values.contains(&v.as_str()))).then_some(*key)
    })
}

/// The first parameter name, at any depth and in any case, that names a password (`password`, `open_password`,
/// `userPassword`…).
fn password_key(params: &Value) -> Option<String> {
    find_key(params, &|key, _| key.contains("password") || key.contains("passphrase"))
}

/// The arguments of `tool` for Nora's `params`: an object, whose `doc` (when the tool takes one) is the active document.
fn arguments(app: &PdfCraftApp, tool: &ToolDef, command: &str, params: &Value) -> Result<Value, String> {
    let mut args = match params {
        Value::Null => Map::new(),
        Value::Object(map) => map.clone(),
        _ => return Err(fmt(tl!("The parameters of `{command}` must be an object."), &[("command", command)])),
    };
    if tool.input_schema.get("properties").and_then(|p| p.get("doc")).is_some() {
        let (_, id) = app.active_ids().ok_or_else(no_document)?;
        if args.get("doc").is_some_and(|doc| !doc.is_null() && doc.as_u64() != Some(id.0)) {
            return Err(tl!("Nora works in the document open in this window: `doc` names another one.").to_string());
        }
        args.insert("doc".into(), json!(id.0));
    }
    Ok(Value::Object(args))
}

/// Runs a tool in the window's documents: the window's session is swapped into the tools for the call and back out after
/// it, whatever happens. A tool that panics is reported, and the documents are kept.
fn call(app: &mut PdfCraftApp, name: &str, args: &Value) -> Result<Vec<Content>, String> {
    std::mem::swap(app.nora.automation.session_mut(), &mut app.session);
    let automation = &mut app.nora.automation;
    let result = pdfcraft_engine::guard(|| automation.call(name, args));
    std::mem::swap(app.nora.automation.session_mut(), &mut app.session);
    match result {
        Ok(Ok(contents)) => Ok(contents),
        Ok(Err(error)) => Err(error.to_string()),
        Err(panic) => {
            Err(fmt(tl!("`{command}` failed unexpectedly ({error}); the document was not changed."), &[("command", name), ("error", &panic)]))
        }
    }
}

/// What a tool answered, as Nora reads it: its JSON (without the paths of the person's files), and a picture as a
/// base64 PNG.
fn output(contents: Vec<Content>) -> Value {
    use base64::Engine as _;
    let mut values: Vec<Value> = contents
        .into_iter()
        .map(|content| match content {
            Content::Json(value) => without_paths(value, 0),
            Content::Png { data, width, height } => {
                json!({ "png": base64::engine::general_purpose::STANDARD.encode(data), "width": width, "height": height })
            }
        })
        .collect();
    if values.len() == 1
        && let Some(value) = values.pop()
    {
        return value;
    }
    json!({ "contents": values })
}

/// `value` without the paths of files (`"path": "/Users/…/contract.pdf"`): the person's folders are none of the server's
/// business. A bookmark's path, a list of numbers, stays.
fn without_paths(value: Value, depth: usize) -> Value {
    match value {
        Value::Object(map) if depth < MAX_DEPTH => Value::Object(
            map.into_iter().filter(|(key, value)| !(key == "path" && value.is_string())).map(|(k, v)| (k, without_paths(v, depth + 1))).collect(),
        ),
        Value::Array(list) if depth < MAX_DEPTH => Value::Array(list.into_iter().map(|v| without_paths(v, depth + 1)).collect()),
        other => other,
    }
}

/// Several commands in order. The first that fails ends the batch with an error, and the document goes back to how it
/// was before the batch (see [`change`]).
fn batch(app: &mut PdfCraftApp, commands: &[CommandCall]) -> Result<Value, String> {
    if commands.len() > MAX_BATCH {
        return Err(fmt(tl!("Commands in one batch: {count} at most."), &[("count", &MAX_BATCH.to_string())]));
    }
    let mut results = Vec::new();
    for call in commands {
        match execute(app, &call.command, &call.params) {
            Ok(value) => results.push(json!({ "command": call.command, "ok": true, "result": value })),
            Err(error) => return Err(fmt(tl!("`{command}` failed: {error}"), &[("command", &call.command), ("error", &error)])),
        }
    }
    Ok(json!({ "results": results }))
}

/// A text extractor for the pages of `doc`, made when a page's text isn't at hand already.
struct Pages<'a> {
    doc: &'a Document,
    view: &'a crate::DocView,
    renderer: Option<PageRenderer>,
}

impl<'a> Pages<'a> {
    fn new(doc: &'a Document, view: &'a crate::DocView) -> Self {
        Pages { doc, view, renderer: None }
    }

    /// The text of `page` (0-based): the one the view has, else read from the document as the view reads it.
    fn text(&mut self, page: usize) -> Option<Arc<PageText>> {
        if let Some(text) = self.view.page_text(page) {
            return Some(text);
        }
        let doc = self.doc;
        let renderer = self.renderer.get_or_insert_with(|| {
            // The layers shown and hidden as the view shows them (the engine's own conversion), so that the glyphs are the
            // view's and a selection lands where it is drawn.
            let layers = doc.info.layers.iter().map(|l| (l.id.0 as i32, l.id.1 as i32, l.visible)).collect();
            let config = RenderConfig {
                password: doc.password.as_deref().map(Arc::from),
                layers: Arc::new(layers),
                hide_comments: doc.comments_hidden(),
                ..RenderConfig::default()
            };
            PageRenderer::new(doc.display.clone(), config)
        });
        let out = renderer.render(RenderRequest { page, kind: RequestKind::Text, tile: None, scale: 1.0, tag: 0, preview: false });
        if out.error.is_some() {
            return None;
        }
        out.text
    }
}

/// The order Nora reads `count` pages in: the visible ones (0-based), then the others in order. With none visible, the
/// current one first.
fn reading_order(count: usize, visible: &[usize], current: usize) -> Vec<usize> {
    let mut first: Vec<usize> = visible.iter().copied().filter(|p| *p < count).collect();
    first.sort_unstable();
    first.dedup();
    if first.is_empty() && current < count {
        first.push(current);
    }
    let rest = (0..count).filter(|p| !first.contains(p));
    first.iter().copied().chain(rest).collect()
}

/// The text Nora reads: each page under a line that names it (`Page 2 (current, visible):`), pages a blank line apart,
/// cut at `max_chars` characters and `max_pages` pages. Also how many pages were read, and whether anything was left out.
fn assemble(
    pages: impl Iterator<Item = (usize, String)>,
    count: usize,
    current: usize,
    visible: &[usize],
    max_chars: usize,
    max_pages: usize,
) -> (String, usize, bool) {
    let mut text = String::new();
    let mut chars = 0;
    let mut read = 0;
    let mut cut = false;
    for (page, page_text) in pages {
        if read >= max_pages || chars >= max_chars {
            cut = true;
            break;
        }
        let marks: Vec<&str> =
            [(page == current, "current"), (visible.contains(&page), "visible")].into_iter().filter(|(on, _)| *on).map(|(_, m)| m).collect();
        let marks = if marks.is_empty() { String::new() } else { format!(" ({})", marks.join(", ")) };
        let block = format!("{}Page {}{marks}:\n{}", if read == 0 { "" } else { "\n\n" }, page + 1, page_text.trim_end());
        let room = max_chars - chars;
        let length = block.chars().count();
        if length > room {
            text.extend(block.chars().take(room));
            read += 1;
            cut = true;
            break;
        }
        text.push_str(&block);
        chars += length;
        read += 1;
    }
    (text, read, cut || read < count)
}

/// What Nora learns when she reads the document: its name, its page count, the current page and the visible ones, the
/// text of its pages (the visible ones first, cut at a length that fits her), and the selected text. A document whose
/// security forbids copying its text gives none of it.
fn observe(app: &mut PdfCraftApp) -> Result<Value, String> {
    let (i, id) = app.active_ids().ok_or_else(no_document)?;
    let (Some(doc), Some(view)) = (app.session.get(id), app.views.get(i)) else { return Err(no_document()) };
    let count = doc.info.pages.len();
    let current = view.current.min(count.saturating_sub(1));
    let visible: Vec<usize> = view.visible_page_rects().into_iter().map(|(page, _)| page).filter(|page| *page < count).collect();
    let order = reading_order(count, &visible, current);
    let allowed = doc.allows_copying();
    let (text, read, truncated) = if allowed {
        let mut pages = Pages::new(doc, view);
        let texts = order.into_iter().map(|page| (page, pages.text(page).map(|t| t.plain_text()).unwrap_or_default()));
        assemble(texts, count, current, &visible, MAX_OBSERVED_CHARS, MAX_OBSERVED_PAGES)
    } else {
        (String::new(), 0, count > 0)
    };
    let selection: String =
        if allowed { view.selected_text().unwrap_or_default().chars().take(MAX_SELECTION_CHARS).collect() } else { String::new() };
    let mut visible: Vec<usize> = visible.into_iter().map(|page| page + 1).collect();
    visible.sort_unstable();
    visible.dedup();
    let mut seen = json!({
        "name": doc.name,
        "pages": count,
        "currentPage": if count == 0 { 0 } else { current + 1 },
        "visiblePages": visible,
        "pagesRead": read,
        "text": text,
        "textTruncated": truncated,
        "selection": { "text": selection },
        "editable": doc.read_only_reason.is_none(),
        "unsaved": doc.dirty,
    });
    if !allowed && let Some(map) = seen.as_object_mut() {
        map.insert("textRefused".into(), json!(crate::copy_refused()));
    }
    Ok(seen)
}

/// Selects the `occurrence`-th time `needle` appears in the document, pages in order, as written, the way the person
/// selects text on a page, and brings its page into view.
fn select_text(app: &mut PdfCraftApp, needle: &str, occurrence: u32) -> Result<Value, String> {
    let (i, id) = app.active_ids().ok_or_else(no_document)?;
    // Where and how often a text occurs would spell out a document that forbids copying it.
    if forbids_copying(app) {
        return Err(no_copying("doc_select_text"));
    }
    let wanted = usize::try_from(occurrence).unwrap_or(usize::MAX).max(1);
    let found = {
        let (Some(doc), Some(view)) = (app.session.get(id), app.views.get(i)) else { return Err(no_document()) };
        let count = doc.info.pages.len();
        let mut pages = Pages::new(doc, view);
        let mut seen = 0usize;
        let mut found = None;
        for page in 0..count.min(MAX_SEARCHED_PAGES) {
            let Some(text) = pages.text(page) else { continue };
            for range in text.find_opts(needle, true, false) {
                seen += 1;
                if seen == wanted && !range.is_empty() {
                    found = Some((page, range, text.clone()));
                    break;
                }
            }
            if found.is_some() {
                break;
            }
        }
        match found {
            Some(found) => found,
            None if seen > 0 => {
                return Err(fmt(
                    tl!("Occurrences of “{text}” in the document: {count} (not {occurrence})."),
                    &[("text", needle), ("count", &seen.to_string()), ("occurrence", &wanted.to_string())],
                ));
            }
            None if count > MAX_SEARCHED_PAGES => {
                return Err(fmt(
                    tl!("“{text}” is not in the first {pages} pages of the document."),
                    &[("text", needle), ("pages", &MAX_SEARCHED_PAGES.to_string())],
                ));
            }
            None => return Err(fmt(tl!("“{text}” is not in the document."), &[("text", needle)])),
        }
    };
    let (page, range, text) = found;
    let view = app.views.get_mut(i).ok_or_else(no_document)?;
    view.texts.entry(page).or_insert(text);
    view.select_text(page, range.start, range.end.saturating_sub(1));
    view.go_to_page(page);
    // What Nora asked for, not the page's text: a document that forbids copying its text gives none of it.
    Ok(json!({ "page": page + 1, "text": needle, "occurrence": wanted }))
}

/// Types Nora's text: into the form field being filled (its text editor open without the keyboard: the person is not
/// typing in it, or Nora would be waiting), else into a new text box comment on the current page, in the first free
/// place (never over the page's text, its comments or its fields), else at the top of the page. A single-line field
/// takes the lines on one line.
fn type_text(app: &mut PdfCraftApp, text: &str, _paragraphs: bool) -> Result<Value, String> {
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    if text.trim().is_empty() {
        return Err(tl!("There is no text to type.").to_string());
    }
    let (i, id) = app.active_ids().ok_or_else(no_document)?;
    if let Some(typed) = type_in_field(app, i, id, &text)? {
        return Ok(typed);
    }
    let (page, rect, free) = {
        let (Some(doc), Some(view)) = (app.session.get(id), app.views.get(i)) else { return Err(no_document()) };
        if doc.info.pages.is_empty() {
            return Err(no_document());
        }
        let page = view.current.min(doc.info.pages.len().saturating_sub(1));
        let page_text = Pages::new(doc, view).text(page);
        let (rect, free) = place_text_box(doc, page_text.as_deref(), page, &text);
        (page, rect, free)
    };
    let author = app.comment_prefs.author.clone();
    let args = json!({
        "doc": id.0,
        "page": page + 1,
        "type": "textbox",
        "rect": rect,
        "contents": text,
        "author": author,
        "font_size": TEXT_SIZE,
    });
    let added = call(app, "comment_add", &args).map(output)?;
    let mut typed = json!({ "page": page + 1, "textBox": true, "rect": rect, "freeSpace": free });
    // What the box stored as typed but draws with `?` (its font can't draw it): said, not silent.
    if let Some(warnings) = added.get("warnings") {
        typed["warnings"] = warnings.clone();
    }
    Ok(typed)
}

/// Fills the text field whose editor is open, when one is: `None` otherwise.
fn type_in_field(app: &mut PdfCraftApp, i: usize, id: DocId, text: &str) -> Result<Option<Value>, String> {
    let Some(focus) = app.views.get(i).and_then(|v| v.forms.focus.clone()) else { return Ok(None) };
    let Some(field) = app.session.get(id).and_then(|d| d.form.iter().find(|f| f.name == focus.name).cloned()) else { return Ok(None) };
    if field.kind != FormFieldKind::Text || field.read_only() {
        return Ok(None);
    }
    let value = if field.has(field_flags::MULTILINE) { text.to_string() } else { text.split('\n').collect::<Vec<_>>().join(" ") };
    if let Some(view) = app.views.get_mut(i) {
        view.forms.focus = None;
    }
    if let Err(e) = app.session.apply(id, Edit::SetFieldValue { name: field.name.clone(), value: FieldValue::Text(value) }) {
        if let Some(view) = app.views.get_mut(i) {
            view.forms.focus = Some(focus);
        }
        return Err(e.to_string());
    }
    let page = field.widgets.get(focus.widget).and_then(|w| w.page).map(|p| p + 1);
    let mut typed = json!({ "field": field.name, "page": page });
    // What the field stored as typed but draws with `?` (its font can't draw it): said, not silent.
    let notices: Vec<String> = app.session.get(id).map(|d| d.edit_notices.iter().map(ToString::to_string).collect()).unwrap_or_default();
    if !notices.is_empty() {
        typed["warnings"] = json!(notices);
    }
    Ok(Some(typed))
}

/// Where Nora's text box goes on `page` (0-based), in points from the top-left of the displayed page: the first place,
/// top to bottom and left to right inside the page's margins, that touches no text, comment or form field of the page;
/// else the top-left corner inside the margins, where it goes when nothing says where. Also whether the place is free.
fn place_text_box(doc: &Document, text: Option<&PageText>, page: usize, contents: &str) -> ([f32; 4], bool) {
    let Some(info) = doc.info.pages.get(page) else { return ([MARGIN, MARGIN, MARGIN + 120.0, MARGIN + 24.0], false) };
    let (width, height) = (info.width.max(1.0), info.height.max(1.0));
    let (w, h) = text_box_size(contents, (width - 2.0 * MARGIN).max(48.0));
    let view_rect = |r: [f32; 4]| {
        let ([x0, y0], [x1, y1]) = (info.user_to_view(r[0], r[1]), info.user_to_view(r[2], r[3]));
        [x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)]
    };
    let mut taken: Vec<[f32; 4]> = text.map(line_boxes).unwrap_or_default();
    taken.extend(doc.info.annotations.iter().filter(|a| a.page == page).map(|a| view_rect(a.rect)));
    taken.extend(
        doc.form
            .iter()
            .flat_map(|f| f.widgets.iter())
            .filter(|w| w.page == Some(page) && !w.hidden)
            .map(|w| view_rect([w.rect[0] as f32, w.rect[1] as f32, w.rect[2] as f32, w.rect[3] as f32])),
    );
    let fallback = [MARGIN, MARGIN, (MARGIN + w).min(width), (MARGIN + h).min(height)];
    match free_place(&taken, width, height, w, h) {
        Some(rect) => (rect, true),
        None => (fallback, false),
    }
}

/// The box of each line of a page's text, in view space: past [`MAX_TAKEN`] lines, one more, which tells the search for
/// free space to give up (a page made of thousands of lines can't hold the window).
fn line_boxes(text: &PageText) -> Vec<[f32; 4]> {
    let mut boxes: Vec<[f32; 4]> = Vec::new();
    let mut index: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    for (glyph, line) in text.glyphs.iter().zip(text.line_of.iter()) {
        let r = glyph.rect;
        match index.get(line).copied() {
            Some(n) => {
                if let Some(b) = boxes.get_mut(n) {
                    *b = [b[0].min(r[0]), b[1].min(r[1]), b[2].max(r[2]), b[3].max(r[3])];
                }
            }
            None if boxes.len() > MAX_TAKEN => break,
            None => {
                index.insert(*line, boxes.len());
                boxes.push(r);
            }
        }
    }
    boxes
}

/// The size of a text box for `contents` in [`TEXT_SIZE`] points, at most `max_width` wide (longer lines wrap).
fn text_box_size(contents: &str, max_width: f32) -> (f32, f32) {
    let per_char = TEXT_SIZE * 0.55;
    let line_height = TEXT_SIZE * 1.35;
    let mut lines = 0.0f32;
    let mut widest = 0.0f32;
    for line in contents.split('\n') {
        let natural = line.chars().count() as f32 * per_char + 8.0;
        widest = widest.max(natural.min(max_width));
        lines += (natural / max_width).ceil().max(1.0);
    }
    (widest.clamp(96.0_f32.min(max_width), max_width), (lines * line_height + 8.0).max(line_height + 8.0))
}

/// The first `w`×`h` place, top to bottom and left to right inside the margins of a `width`×`height` page, that stays
/// [`GAP`] away from every rectangle of `taken`: tried at most [`GAP`] points apart, edges included, or at [`MAX_TRIES`]
/// steps along a side of a huge page.
fn free_place(taken: &[[f32; 4]], width: f32, height: f32, w: f32, h: f32) -> Option<[f32; 4]> {
    let (room_x, room_y) = (width - 2.0 * MARGIN - w, height - 2.0 * MARGIN - h);
    if taken.len() > MAX_TAKEN || !(room_x >= 0.0 && room_y >= 0.0 && room_x.is_finite() && room_y.is_finite()) {
        return None;
    }
    let fits = |r: [f32; 4]| taken.iter().all(|t| r[2] + GAP <= t[0] || t[2] + GAP <= r[0] || r[3] + GAP <= t[1] || t[3] + GAP <= r[1]);
    // How many steps along a side, and where the k-th of n falls (the last on the edge itself).
    let steps = |room: f32| ((room / GAP).ceil().min(MAX_TRIES as f32) as usize).max(1);
    let at = |room: f32, k: usize, n: usize| if k >= n { room } else { room * k as f32 / n as f32 };
    let (across, down) = (steps(room_x), steps(room_y));
    for row in 0..=down {
        let y = MARGIN + at(room_y, row, down);
        for column in 0..=across {
            let x = MARGIN + at(room_x, column, across);
            let r = [x, y, x + w, y + h];
            if fits(r) {
                return Some(r);
            }
        }
    }
    None
}

/// A page as a picture, to look at the result (`render_preview`, a base64 PNG; `scale` 1 is 1,024 pixels on its longer
/// side).
fn render_page(app: &mut PdfCraftApp, page: u32, scale: f64) -> Result<Value, String> {
    let (_, id) = app.active_ids().ok_or_else(no_document)?;
    // A picture of a page is its text too, as the editor's snapshots and exported pictures are.
    if forbids_copying(app) {
        return Err(no_copying("doc_render_page"));
    }
    let count = app.session.get(id).map_or(0, |d| d.info.pages.len());
    if page == 0 || usize::try_from(page).ok().is_none_or(|p| p > count) {
        return Err(fmt(tl!("There is no page {page}: the last page is {count}."), &[("page", &page.to_string()), ("count", &count.to_string())]));
    }
    let scale = if scale.is_finite() { scale.clamp(0.1, 4.0) } else { 1.0 };
    let max_side = (1024.0 * scale).round().clamp(64.0, 4096.0) as u32;
    let mut picture = call(app, "render_preview", &json!({ "doc": id.0, "page": page, "max_side": max_side })).map(output)?;
    if let Some(map) = picture.as_object_mut() {
        map.insert("page".into(), json!(page));
        map.insert("pages".into(), json!(count));
    }
    Ok(picture)
}

/// Saves the document with the app's own Save: to its file, or, for one without a file of its own, through its Save As
/// (the hosting page, a dialog, or a download). The person said yes first.
fn save(app: &mut PdfCraftApp) -> Result<Value, String> {
    let (_, id) = app.active_ids().ok_or_else(no_document)?;
    let name = app.session.get(id).map(|d| d.name.clone()).ok_or_else(no_document)?;
    let shown = app.toast.take();
    let written = app.save_active(crate::SaveTarget::InPlace);
    let said = app.toast.as_ref().map(|(message, _)| message.clone());
    if app.toast.is_none() {
        app.toast = shown;
    }
    let unsaved = app.session.get(id).is_some_and(|d| d.dirty);
    if written || !unsaved {
        return Ok(json!({ "saved": true, "name": name }));
    }
    match said {
        Some(why) => Err(fmt(tl!("{name} was not saved: {why}"), &[("name", &name), ("why", &why)])),
        // The person is asked where to save it; the answer comes later, from them.
        None => Ok(json!({ "saved": false, "name": name, "asking": "the person chooses where to save it" })),
    }
}

/// Takes back the last change Nora made, whole, while the document is as she left it: had the person changed it since,
/// the same undo steps would take back their work.
fn undo(app: &mut PdfCraftApp) -> Result<Value, String> {
    let Some(change) = app.nora.changes.pop() else { return Err(tl!("Nora has nothing left to undo.").to_string()) };
    if revision_of(app) != Some((change.doc, change.after)) {
        // Nora's earlier changes lie under the person's too.
        app.nora.changes.clear();
        return Err(tl!("The document has changed since Nora's last change: use Undo in the editor instead.").to_string());
    }
    // The editor keeps so many undo steps: a change whose first steps fell out of them can't be taken back whole (nor
    // can the ones before it).
    if app.session.get(change.doc).map_or(0, Document::undo_depth) < change.steps {
        app.nora.changes.clear();
        return Err(tl!("Nora's change is too old to be taken back whole: use Undo in the editor instead.").to_string());
    }
    for done in 0..change.steps {
        if let Err(e) = app.session.undo(change.doc) {
            app.nora.changes.clear();
            refresh(app, change.doc);
            return Err(fmt(
                tl!("Undo failed at step {step} of {count}: {error}"),
                &[("step", &(done + 1).to_string()), ("count", &change.steps.to_string()), ("error", &e.to_string())],
            ));
        }
    }
    refresh(app, change.doc);
    // The change before it can be undone in turn when nothing came between the two.
    let now = revision_of(app);
    if let (Some(previous), Some((id, generation))) = (app.nora.changes.last_mut(), now)
        && previous.doc == id
        && previous.after == change.before
    {
        previous.after = generation;
    }
    Ok(json!({ "undone": change.steps }))
}

/// Hands one call to the host. Without a connection the call fails at once, as an unavailable server.
fn send(app: &mut PdfCraftApp, request: HttpRequest) -> Vec<Output> {
    match &app.nora_http {
        Some(http) => {
            http(request);
            Vec::new()
        }
        None => app.nora.session.handle(Input::Http(HttpResponse { id: request.id, status: 503, body: String::new() })),
    }
}

/// Where the person is: the editor, the document's name and page count, and the selected text (none from a document whose
/// security forbids copying its text).
fn context_of(app: &PdfCraftApp) -> PageContext {
    let open = app.active_ids().and_then(|(i, id)| Some((app.session.get(id)?, app.views.get(i)?)));
    let Some((doc, view)) = open else { return PageContext::editor(AppKind::Pdf, DocumentInfo::named("")) };
    let selection = view
        .selected_text()
        .filter(|text| !text.trim().is_empty() && doc.allows_copying())
        .map(|text| Selection { text: text.chars().take(MAX_SELECTION_CHARS).collect() });
    let pages = u32::try_from(doc.info.pages.len()).ok();
    PageContext::editor(AppKind::Pdf, DocumentInfo { name: doc.name.clone(), id: None, pages, selection })
}

/// The pane's bubbles in PdfCraft's colours, light or dark: the person's messages on the accent, in the ink that reads
/// on it (the account button's), Nora's on the hover grey.
fn style(t: &Tokens) -> NoraStyle {
    NoraStyle {
        user_fill: Some(t.accent),
        user_text: Some(crate::account::ink(t)),
        nora_fill: Some(t.hover),
        nora_text: Some(t.text),
        ..NoraStyle::default()
    }
}

/// The words of the pane in the interface language. The pane's title row is PdfCraft's own, so the panel's is left empty.
fn labels() -> NoraLabels {
    NoraLabels {
        title: String::new(),
        new_conversation: tl!("New conversation").to_string(),
        undo: tl!("Undo").to_string(),
        undo_hint: tl!("Undo the last change Nora made to the document").to_string(),
        placeholder: tl!("Ask Nora…").to_string(),
        send: tl!("Send").to_string(),
        stop: tl!("Stop").to_string(),
        working: tl!("Nora is working…").to_string(),
        stopped: tl!("Stopped.").to_string(),
        empty: tl!("Ask Nora to read, summarize, fill in or annotate your document.").to_string(),
        confirm_heading: tl!("Nora wants to:").to_string(),
        allow: tl!("Allow").to_string(),
        refuse: tl!("Refuse").to_string(),
        describe,
    }
}

/// A failure of the connection to Nora in the interface language (the panel shows the editor's own sentences, an action
/// it refused, as they come). The server's own words, when it gave some, are shown as they are: the server writes them
/// for the person.
fn error_words(failure: &NoraError) -> String {
    match failure {
        NoraError::Timeout => tl!("Nora did not answer in time.").to_string(),
        NoraError::Unreachable(_) => tl!("Nora could not be reached. Check the connection and try again.").to_string(),
        NoraError::Http(401 | 403) => tl!("Your session has expired. Sign in again.").to_string(),
        NoraError::Http(status) => {
            fmt(tl!("The server could not answer (error {status}). Try again in a moment."), &[("status", &status.to_string())])
        }
        NoraError::Server { message, .. } => message.clone(),
        NoraError::Malformed(_) => tl!("Nora's answer could not be read.").to_string(),
    }
}

/// One sentence for an action waiting for the person's yes, in the interface language.
fn describe(kind: &DocActionKind) -> String {
    match kind {
        DocActionKind::Observe => tl!("Read the document").to_string(),
        DocActionKind::Execute { command, .. } => fmt(tl!("Run {command}"), &[("command", command)]),
        DocActionKind::Batch { commands } => match commands.as_slice() {
            [only] => fmt(tl!("Run {command}"), &[("command", &only.command)]),
            _ => fmt(tl!("Run several commands ({count})"), &[("count", &commands.len().to_string())]),
        },
        DocActionKind::SelectText { text, .. } => fmt(tl!("Select “{text}”"), &[("text", &shorten(text))]),
        DocActionKind::TypeText { text, .. } => fmt(tl!("Type “{text}”"), &[("text", &shorten(text))]),
        DocActionKind::RenderPage { page, .. } => fmt(tl!("Look at page {page}"), &[("page", &page.to_string())]),
        DocActionKind::Save => tl!("Save the document").to_string(),
        DocActionKind::Undo => tl!("Undo the last change").to_string(),
    }
}

/// The start of a text, for a sentence that quotes it.
fn shorten(text: &str) -> String {
    let mut short: String = text.chars().take(40).collect();
    if text.chars().count() > 40 {
        short.push('…');
    }
    short
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::*;
    use crate::{Dialog, NoraInbox};

    type Outbox = Arc<Mutex<Vec<HttpRequest>>>;
    type Window = Harness<'static, PdfCraftApp>;

    /// A PDF of 300 × 400 point pages; page `i` shows the lines of `pages[i]` in Helvetica 12, 20 points apart from the top.
    fn pdf(pages: &[&[&str]]) -> Vec<u8> {
        let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
        let kids: Vec<String> = (0..pages.len()).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
        objs.push(format!("<< /Type /Pages /Kids [{}] /Count {} /MediaBox [0 0 300 400] >>", kids.join(" "), pages.len()));
        objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".into());
        for (i, lines) in pages.iter().enumerate() {
            objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i));
            let shown: Vec<String> = lines.iter().enumerate().map(|(n, l)| format!("{}({l}) Tj", if n == 0 { "" } else { "0 -20 Td " })).collect();
            let body = format!("BT /F1 12 Tf 30 360 Td {} ET", shown.join(" "));
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

    /// A window, in English, with `bytes` open as `doc.pdf`.
    fn open(bytes: Vec<u8>) -> PdfCraftApp {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.open_bytes("doc.pdf", None, bytes).unwrap();
        app
    }

    /// The document most tests work on: two pages, the first current.
    fn document() -> PdfCraftApp {
        open(pdf(&[&["Hello big world", "one two one"], &["Second page", "one more"]]))
    }

    /// A host that records the calls and lets the test answer them; the pane is open.
    fn connect(app: &mut PdfCraftApp) -> (Outbox, NoraInbox) {
        let outbox: Outbox = Arc::default();
        let inbox: NoraInbox = Arc::default();
        let sent = outbox.clone();
        app.nora_http = Some(Box::new(move |request| sent.lock().unwrap().push(request)));
        app.nora_inbox = Some(inbox.clone());
        app.nora_pane = true;
        (outbox, inbox)
    }

    fn window(app: PdfCraftApp) -> Window {
        let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| app);
        // Fonts install on frame 1 and apply on frame 2.
        h.run_steps(6);
        h
    }

    fn answer(inbox: &NoraInbox, request: &HttpRequest, message: Value) {
        let body = json!({ "message": message }).to_string();
        inbox.lock().unwrap().push(HttpResponse { id: request.id, status: 200, body });
    }

    fn take(outbox: &Outbox) -> Vec<HttpRequest> {
        std::mem::take(&mut *outbox.lock().unwrap())
    }

    /// Sends `message` from the pane, gives the conversation an id, and returns the reads that follow.
    fn start(h: &mut Window, outbox: &Outbox, inbox: &NoraInbox, message: &str) -> Vec<HttpRequest> {
        h.state_mut().nora.panel.set_draft(message);
        h.run_steps(2);
        h.get_by_label("Send").click();
        h.run_steps(4);
        let first = take(outbox);
        answer(inbox, &first[0], json!({ "thread_id": "t1" }));
        h.run_steps(6);
        take(outbox)
    }

    /// Hands Nora's `actions` to the pane's reading of its action queue, and returns what the pane said back.
    fn act(h: &mut Window, outbox: &Outbox, inbox: &NoraInbox, polls: &[HttpRequest], actions: Value) -> Vec<HttpRequest> {
        let read = polls.iter().find(|r| r.path.ends_with("get_browser_actions")).expect("the actions are read");
        answer(inbox, read, json!({ "actions": actions }));
        h.run_steps(4);
        take(outbox)
    }

    fn observation<'a>(told: &'a [HttpRequest], callback: &str) -> &'a Value {
        let call = told.iter().find(|r| r.path.ends_with("receive_observation") && r.body["callback_id"] == callback);
        &call.unwrap_or_else(|| panic!("Nora gets an answer for {callback}: {told:?}")).body["observation"]
    }

    fn run(app: &mut PdfCraftApp, kind: DocActionKind) -> Result<Value, String> {
        run_document(app, &kind)
    }

    fn exec(command: &str, params: Value) -> DocActionKind {
        DocActionKind::Execute { command: command.into(), params }
    }

    fn step(command: &str, params: Value) -> CommandCall {
        CommandCall { command: command.into(), params }
    }

    fn doc(app: &PdfCraftApp) -> &Document {
        app.session.get(app.views[0].id).unwrap()
    }

    fn rotation(app: &PdfCraftApp, page: usize) -> u16 {
        doc(app).info.pages[page].rotation
    }

    fn rotate(page: usize) -> DocActionKind {
        exec("page_rotate", json!({ "pages": [page], "degrees": 90 }))
    }

    /// The person's own edit, as the interface makes it.
    fn person_rotates(app: &mut PdfCraftApp, page: usize) {
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![page], degrees: 90 }));
    }

    #[test]
    fn nora_is_a_command_and_a_preference_kept_between_runs() {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        assert!(!app.nora_pane, "off by default");
        let spec = pdfcraft_engine::commands::command("ui.nora").expect("registered");
        assert_eq!((spec.label, spec.menu), ("Nora assistant", None), "in the palette, not in upstream's menus");
        assert!(app.execute("ui.nora"));
        assert!(app.nora_pane, "the command toggles it");
        assert!(app.execute("ui.nora"));
        assert!(!app.nora_pane);
        app.set_option("nora", "on").unwrap();
        assert!(app.nora_pane);
        assert!(app.set_option("nora", "maybe").is_err());
        assert!(app.nora_pane, "a rejected value keeps the setting");
        let mut restored = PdfCraftApp::new();
        restored.restore(&app.persist());
        assert!(restored.nora_pane, "kept between runs");
        let mut older = PdfCraftApp::new();
        older.restore(r#"{"theme": "Dark"}"#);
        assert!(!older.nora_pane, "settings saved before the pane existed leave it off");
        app.set_option("nora", "off").unwrap();
        assert!(!app.nora_pane);
    }

    #[test]
    fn the_preferences_switch_shows_and_hides_the_pane() {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.dialog = Some(Dialog::Preferences);
        let mut h = window(app);
        assert!(h.query_by_label("Assistant").is_some(), "the section is there");
        h.get_by_label("Show the Nora assistant pane (experimental)").click_accesskit();
        h.run_steps(4);
        assert!(h.state().nora_pane, "the checkbox turns it on");
        h.get_by_label("Show the Nora assistant pane (experimental)").click_accesskit();
        h.run_steps(4);
        assert!(!h.state().nora_pane, "and off again");
    }

    /// Nothing is drawn while the pane is off; open, without a connection, it says why it can't be used instead of
    /// offering a text box.
    #[test]
    fn without_a_connection_the_pane_says_nora_isnt_available() {
        let mut h = window(document());
        assert!(h.query_by_label("Nora isn't available in this version yet").is_none(), "off by default");
        assert!(h.query_by_label("Nora").is_none());
        h.state_mut().execute("ui.nora");
        h.run_steps(4);
        assert!(h.query_by_label("Nora").is_some(), "the pane's title");
        assert!(h.query_by_label("Nora isn't available in this version yet").is_some());
        assert!(h.query_by_label("Ask Nora to read, summarize, fill in or annotate your document.").is_some());
        assert!(h.query_by_role(Role::MultilineTextInput).is_none(), "no text box to lose a message in");
        // The close button of its title row, at the right of the window.
        let close = h.get_all_by_label("Close panel").max_by(|a, b| a.rect().center().x.total_cmp(&b.rect().center().x)).unwrap();
        close.click();
        h.run_steps(4);
        assert!(!h.state().nora_pane);
        assert!(h.query_by_label("Nora isn't available in this version yet").is_none(), "closed, it draws nothing");
    }

    /// A message typed in the pane leaves as the first call of the conversation, carrying where the person is; the host's
    /// answers show up in the pane; the document is left alone.
    #[test]
    fn a_connected_pane_sends_the_message_with_its_context_and_shows_the_reply() {
        let mut app = document();
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        let generation = doc(h.state()).edit_generation();
        assert!(h.query_by_label("Nora isn't available in this version yet").is_none());
        h.get_by_role(Role::MultilineTextInput).focus();
        h.run_steps(2);
        h.get_by_role(Role::MultilineTextInput).type_text("Summarize the second page");
        h.run_steps(2);
        h.key_press(egui::Key::Enter);
        h.run_steps(4);

        let calls = take(&outbox);
        assert_eq!(calls.len(), 1, "{calls:?}");
        assert!(calls[0].path.ends_with("nora.api.chat.start_or_resume_thread"), "{}", calls[0].path);
        assert_eq!(calls[0].body["message"], "Summarize the second page");
        let context: Value = serde_json::from_str(calls[0].body["context"].as_str().unwrap_or_default()).unwrap();
        assert_eq!(context["surface"], "editor");
        assert_eq!(context["app"], "pdf");
        assert_eq!(context["document"]["name"], "doc.pdf");
        assert_eq!(context["document"]["pages"], 2);
        assert!(h.query_by_label("Summarize the second page").is_some(), "the person's own message is in the transcript");
        assert_eq!(doc(h.state()).edit_generation(), generation, "typing in the pane never edits the document");

        // The server gives the conversation an id; the pane then reads the replies and the actions.
        answer(&inbox, &calls[0], json!({ "thread_id": "t1", "sent_at": 1000.0 }));
        h.run_steps(6);
        let polls = take(&outbox);
        let reads = polls.iter().find(|r| r.path.ends_with("get_messages")).expect("the replies are read");
        let replies = json!([
            { "role": "assistant", "text": "An answer from an earlier conversation.", "ts": 900.0 },
            { "role": "assistant", "text": "Page 2 says hello — in brief.", "ts": 1001.0 }
        ]);
        answer(&inbox, reads, json!({ "messages": replies }));
        // The keyboard is still in the pane's own text box: that is talking to Nora, not typing in the document, so her
        // actions don't wait for it.
        assert!(h.get_by_role(Role::MultilineTextInput).is_focused());
        let told = act(&mut h, &outbox, &inbox, &polls, json!([{ "action": "doc_observe", "callback_id": "c1" }]));
        assert_eq!(observation(&told, "c1")["result"]["pages"], 2);
        assert!(h.query_by_label("Page 2 says hello — in brief.").is_some(), "Nora's reply is shown");
        assert!(h.query_by_label("An answer from an earlier conversation.").is_none(), "a reply older than the send is not");
    }

    /// What Nora asks for in the document is played, and she gets the result by callback.
    #[test]
    fn a_document_action_runs_and_nora_gets_the_observation() {
        let mut app = document();
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        let polls = start(&mut h, &outbox, &inbox, "Write a greeting");
        let told = act(&mut h, &outbox, &inbox, &polls, json!([{ "action": "doc_type_text", "text": "Bonjour tout le monde", "callback_id": "c1" }]));
        let seen = observation(&told, "c1");
        assert_eq!(seen["ok"], true, "{seen}");
        assert_eq!(seen["result"]["textBox"], true);
        assert_eq!(seen["result"]["freeSpace"], true);
        let typed = doc(h.state()).info.annotations.iter().any(|a| a.subtype == "FreeText" && a.contents.as_deref() == Some("Bonjour tout le monde"));
        assert!(typed, "a text box with Nora's text is on the page");
        assert_eq!(h.state().nora.changes.len(), 1, "one change of Nora's to undo");
    }

    /// A command that is the person's never reaches the document nor the interface, and Nora is told why.
    #[test]
    fn a_refused_command_never_runs() {
        let mut app = document();
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        let generation = doc(h.state()).edit_generation();
        let polls = start(&mut h, &outbox, &inbox, "Close your pane, go dark, save and undo");
        let told = act(
            &mut h,
            &outbox,
            &inbox,
            &polls,
            json!([
                { "action": "doc_execute", "command": "ui.nora", "callback_id": "c1" },
                { "action": "doc_execute", "command": "view.theme.dark", "callback_id": "c2" },
                { "action": "doc_execute", "command": "doc_save", "params": {}, "callback_id": "c3" },
                { "action": "doc_batch", "commands": [{ "command": "page_rotate", "params": { "pages": [1], "degrees": 90 } }, { "command": "edit_undo" }], "callback_id": "c4" }
            ]),
        );
        for callback in ["c1", "c2", "c3", "c4"] {
            let seen = observation(&told, callback);
            assert_eq!(seen["ok"], false, "{callback}: {seen}");
            assert!(seen["error"].as_str().unwrap_or_default().contains("refused"), "{callback}: {seen}");
        }
        assert!(h.state().nora_pane, "the pane is the person's");
        assert_eq!(h.state().theme_preference, crate::theme::ThemePreference::Light, "and so is the theme");
        assert_eq!(doc(h.state()).edit_generation(), generation, "nothing ran, not even the batch's first command");
    }

    #[test]
    fn nora_reads_the_text_of_the_pages_the_visible_ones_first() {
        let mut app = document();
        app.views[0].go_to_page(1);
        let seen = run(&mut app, DocActionKind::Observe).unwrap();
        assert_eq!(seen["text"], "Page 2 (current):\nSecond page\none more\n\nPage 1:\nHello big world\none two one");
        assert_eq!((seen["pages"].clone(), seen["currentPage"].clone(), seen["pagesRead"].clone()), (json!(2), json!(2), json!(2)));
        assert_eq!(seen["textTruncated"], false);
        assert_eq!(seen["name"], "doc.pdf");
        assert_eq!(seen["editable"], true);
        assert!(seen.get("changed").is_none(), "a reading is no change");
        assert!(app.nora.changes.is_empty());
    }

    #[test]
    fn the_pages_on_screen_come_first_then_the_others_in_order() {
        assert_eq!(reading_order(5, &[3, 2, 9], 0), [2, 3, 0, 1, 4]);
        assert_eq!(reading_order(3, &[], 2), [2, 0, 1], "with none on screen, the current page first");
        assert_eq!(reading_order(0, &[], 0), Vec::<usize>::new());
        let (text, read, cut) = assemble([(1, "b".to_string()), (0, "a".to_string())].into_iter(), 2, 1, &[1], 100, 10);
        assert_eq!((text.as_str(), read, cut), ("Page 2 (current, visible):\nb\n\nPage 1:\na", 2, false));
    }

    #[test]
    fn what_nora_reads_is_cut_at_a_length_that_fits_her() {
        let long = "é".repeat(MAX_OBSERVED_CHARS + 10);
        let (text, read, cut) = assemble(std::iter::once((0, long)), 1, 0, &[], MAX_OBSERVED_CHARS, MAX_OBSERVED_PAGES);
        assert_eq!(text.chars().count(), MAX_OBSERVED_CHARS);
        assert_eq!((read, cut), (1, true));
        // And at a number of pages.
        let pages = (0..5).map(|p| (p, format!("text {p}")));
        let (text, read, cut) = assemble(pages, 5, 0, &[], MAX_OBSERVED_CHARS, 2);
        assert_eq!((read, cut), (2, true));
        assert!(text.contains("text 1") && !text.contains("text 2"), "{text}");
        // A real document of more pages than one reading takes says so.
        let many: Vec<&[&str]> = vec![&["x"]; MAX_OBSERVED_PAGES + 3];
        let mut app = open(pdf(&many));
        let seen = run(&mut app, DocActionKind::Observe).unwrap();
        assert_eq!((seen["pagesRead"].clone(), seen["textTruncated"].clone()), (json!(MAX_OBSERVED_PAGES), json!(true)));
    }

    #[test]
    fn a_ui_command_and_a_command_reaching_a_file_or_a_password_are_refused() {
        let mut app = document();
        let generation = doc(&app).edit_generation();
        for (command, params) in [
            ("ui.nora", json!({})),
            ("view.theme.dark", json!({})),
            ("edit.undo", json!({})),
            ("edit_redo", json!({})),
            ("page.copy", json!({})),
            ("link_open", json!({ "page": 1, "index": 1 })),
            ("js_run", json!({ "script": "app.alert(1)" })),
            ("command_run", json!({ "id": "page.rotate" })),
            ("doc_open", json!({ "path": "/etc/hosts" })),
            ("comments_hide", json!({ "hidden": true })),
        ] {
            let error = run(&mut app, exec(command, params)).unwrap_err();
            assert!(error.contains("refused"), "{command}: {error}");
        }
        for params in [
            json!({ "pages": [1], "degrees": 90, "path": "/etc/hosts" }),
            json!({ "pages": [1], "degrees": 90, "options": { "FileName": "x" } }),
            json!({ "pages": [1], "degrees": 90, "list": [{ "DIRECTORY": "/" }] }),
            json!({ "pages": [1], "degrees": 90, "out": "copy.pdf" }),
        ] {
            let error = run(&mut app, exec("page_rotate", params.clone())).unwrap_err();
            assert!(error.contains("would reach a file of the computer"), "{params}: {error}");
        }
        let error = run(&mut app, exec("content_update", json!({ "page": 1, "index": 1, "image": "/photo.png" }))).unwrap_err();
        assert!(error.contains("`image` would reach a file"), "{error}");
        let error = run(&mut app, exec("image_edit", json!({ "page": 1, "image": 1, "action": "delete" }))).unwrap_err();
        assert!(!error.contains("would reach a file"), "a picture's number is no file: {error}");
        for params in [json!({ "open_password": "secret" }), json!({ "settings": { "UserPassword": "secret" } })] {
            let error = run(&mut app, exec("doc_protect", params.clone())).unwrap_err();
            assert!(error.contains("is a password"), "{params}: {error}");
        }
        assert_eq!(doc(&app).edit_generation(), generation, "nothing ran");
        // In a batch, a refused command ends it, and what ran before it is put back.
        let commands =
            vec![step("page_rotate", json!({ "pages": [1], "degrees": 90 })), step("page_add_image", json!({ "page": 1, "path": "x.png" }))];
        let error = run(&mut app, DocActionKind::Batch { commands }).unwrap_err();
        assert!(error.contains("would reach a file"), "{error}");
        assert_eq!(rotation(&app, 0), 0, "the batch's first command was put back");
        // `dir` (a direction elsewhere) names no file, and neither does a name that only starts like one.
        assert_eq!(file_key(&json!({ "dir": "up", "path_index": [1, 2], "file_page": 1 })), None);
        assert!(app.nora.changes.is_empty());
    }

    /// Fill & Sign's typed signature and initials are the person's (its text, marks and dates are Nora's), and the
    /// numbers that lead to a bookmark are no file.
    #[test]
    fn a_signature_is_the_person_s_and_a_bookmark_s_path_is_no_file() {
        let mut app = document();
        for kind in ["signature", " Initials "] {
            let error = run(&mut app, exec("fill_sign_add", json!({ "page": 1, "at": [100, 100], "type": kind, "text": "Jeanne" }))).unwrap_err();
            assert!(error.contains("your signature and initials are yours to add"), "{kind}: {error}");
        }
        assert!(app.nora.changes.is_empty(), "nothing ran");
        run(&mut app, exec("fill_sign_add", json!({ "page": 1, "at": [100, 100], "type": "text", "text": "Jeanne" }))).unwrap();
        assert_eq!(file_key(&json!({ "path": [1, 2] })), None);
        assert_eq!(file_key(&json!({ "path": [1, "/etc/hosts"] })).as_deref(), Some("path"));
        assert_eq!(file_key(&json!({ "path": { "to": "/etc/hosts" } })).as_deref(), Some("path"));
        let error = run(&mut app, exec("bookmark_rename", json!({ "path": [1], "title": "Intro" }))).unwrap_err();
        assert!(!error.contains("would reach a file"), "{error}");
    }

    /// The tools Nora may run though the registry command they play is the person's: the document's properties (read,
    /// with no path in what she gets back) and Fill & Sign (its signature and initials refused by value). A tool of a new
    /// version that joins them is looked at first.
    #[test]
    fn a_tool_playing_a_command_of_the_person_s_is_one_looked_at() {
        let allowed: Vec<(&str, &str)> = TOOLS
            .iter()
            .filter_map(|t| {
                let command = t.command?;
                (POLICY.check(command) == Decision::Deny && POLICY.check(t.name) != Decision::Deny).then_some((t.name, command))
            })
            .collect();
        assert_eq!(allowed, [("doc_info", "file.properties"), ("fill_sign_add", "sign.fill.text")]);
    }

    /// A document whose security forbids copying its text gives Nora none of it, by reading it or by a tool: a search's
    /// matches and their places would spell it out.
    #[test]
    fn a_document_that_forbids_copying_gives_nora_none_of_its_text() {
        let mut cos = pdfcraft_cos::Document::open(Arc::new(pdf(&[&["Confidential figures"]]))).unwrap();
        cos.set_encryption(&pdfcraft_cos::NewEncryption {
            algorithm: pdfcraft_cos::Algorithm::Aes256,
            user_password: "",
            owner_password: "owner",
            permissions: -1 ^ 0b1_0000,
            encrypt_metadata: true,
            seed: [7; 32],
        })
        .unwrap();
        let mut app = open(pdfcraft_cos::write_full(&cos, &Default::default()).unwrap());
        assert!(!doc(&app).allows_copying());
        for (command, params) in [
            ("text_find", json!({ "query": "figures" })),
            ("text_lines", json!({ "page": 1 })),
            ("text_extract", json!({})),
            ("text_paragraphs", json!({ "page": 1 })),
            ("object_list", json!({ "page": 1 })),
            ("ocr_recognize", json!({ "pages": [1] })),
            ("accessibility_autotag_preview", json!({ "page": 1 })),
            ("doc_audit_redactions", json!({})),
        ] {
            let error = run(&mut app, exec(command, params)).unwrap_err();
            assert!(error.contains("forbids copying its text"), "{command}: {error}");
        }
        for (action, kind) in [
            ("doc_select_text", DocActionKind::SelectText { text: "figures".into(), occurrence: 1 }),
            ("doc_render_page", DocActionKind::RenderPage { page: 1, scale: 1.0 }),
        ] {
            let error = run(&mut app, kind).unwrap_err();
            assert!(error.contains("forbids copying its text") && error.contains(action), "{action}: {error}");
        }
        let seen = run(&mut app, DocActionKind::Observe).unwrap();
        assert!(!seen.to_string().contains("Confidential"), "{seen}");
        // The tool itself refuses every line's text, as it refuses text_extract's.
        let id = app.active_ids().map(|(_, id)| id.0).unwrap();
        assert!(call(&mut app, "text_lines", &json!({ "doc": id, "page": 1 })).is_err());
    }

    #[test]
    fn the_policy_keeps_the_interface_the_files_the_history_and_the_scripts_to_the_person() {
        for refused in [
            "ui.nora",
            "ui.dock.reset",
            "view.theme.dark",
            "view.full_screen",
            "help.website",
            "edit.undo",
            "edit.redo",
            "edit_undo",
            "edit_redo",
            "edit.snapshot",
            "page.copy",
            "page.cut",
            "page.paste",
            "file.save",
            "file.open",
            "doc_save",
            "doc_open",
            "doc_open_revision",
            "doc_close",
            "doc_list",
            "doc_create",
            "doc_combine",
            "doc_split",
            "doc_compare",
            "doc_export_text",
            "doc_print",
            "printers",
            "page_extract",
            "page_insert_file",
            "comments_summarize",
            "comments_hide",
            "view_reading_colors",
            "link_open",
            "js_run",
            "js_enabled",
            "form_set_script",
            "form_set_actions",
            "action_run",
            "command_run",
            "command_batch",
            "sign_document",
            "sign.fill.signature",
            "sign_keychain_ids",
            "text_font_files",
            "create.blank",
            "print.dialog",
            "export.text",
            "tools.js_console",
        ] {
            assert_eq!(POLICY.check(refused), Decision::Deny, "{refused}");
        }
        for allowed in [
            "page_rotate",
            "page.rotate",
            "comment_add",
            "comment_reply",
            "text_extract",
            "text_find",
            "form_fill",
            "doc_set_info",
            "doc_info",
            "redact_mark",
            "bookmark_add",
            "sign_list",
            "print_readiness",
            "render_preview",
            "page_render",
            "link_add",
            "ocr_recognize",
        ] {
            assert_eq!(POLICY.check(allowed), Decision::Allow, "{allowed}");
        }
        // Registry commands that only work through the interface, or that several tools play, name what to do instead.
        let mut app = document();
        assert!(run(&mut app, exec("comment.note", json!({}))).unwrap_err().contains("only through the interface"));
        assert!(run(&mut app, exec("form.prepare", json!({}))).unwrap_err().contains("several tools"));
        assert!(run(&mut app, exec("no.such.command", json!({}))).unwrap_err().contains("not a command of NeoPDF"));
        // A registry command a tool plays is played by it.
        run(&mut app, exec("page.rotate", json!({ "pages": [2], "degrees": 90 }))).unwrap();
        assert_eq!(rotation(&app, 1), 90);
    }

    /// The coordinator's rule: an action that changed nothing says so, so that there is nothing of it to undo.
    #[test]
    fn a_read_only_command_says_it_changed_nothing_and_a_change_does_not() {
        let mut app = document();
        let read = run(&mut app, exec("text_extract", json!({ "pages": [1] }))).unwrap();
        assert_eq!(read["changed"], false, "{read}");
        assert!(read["pages"][0]["text"].as_str().is_some_and(|t| t.contains("Hello big world")), "{read}");
        let batch = run(&mut app, DocActionKind::Batch { commands: vec![step("doc_info", json!({})), step("text_find", json!({ "query": "one" }))] })
            .unwrap();
        assert_eq!(batch["changed"], false, "{batch}");
        assert_eq!(batch["results"].as_array().map(Vec::len), Some(2));
        assert!(app.nora.changes.is_empty(), "nothing to undo");
        let changed = run(&mut app, rotate(1)).unwrap();
        assert!(changed.get("changed").is_none(), "{changed}");
        let typed = run(&mut app, DocActionKind::TypeText { text: "A note".into(), paragraphs: false }).unwrap();
        assert!(typed.get("changed").is_none(), "{typed}");
        assert_eq!(app.nora.changes.len(), 2);
    }

    #[test]
    fn an_action_that_fails_leaves_the_document_as_it_was() {
        let mut app = document();
        let id = app.views[0].id;
        // A saved document whose last edit the person took back: something to redo, nothing unsaved.
        person_rotates(&mut app, 1);
        app.undo();
        let saved = app.session.save_bytes(id).unwrap();
        app.session.mark_saved(id, saved, None).unwrap();
        let (bytes, redo) = (doc(&app).bytes.clone(), doc(&app).can_redo().map(str::to_owned));
        assert!(!doc(&app).dirty && redo.is_some());

        let commands = vec![
            step("page_rotate", json!({ "pages": [1], "degrees": 90 })),
            step("page_delete", json!({ "pages": [2] })),
            step("no_such_tool", json!({})),
        ];
        let error = run(&mut app, DocActionKind::Batch { commands }).unwrap_err();
        assert!(error.contains("no_such_tool"), "{error}");
        let d = doc(&app);
        assert_eq!(d.bytes, bytes, "the document of before");
        assert_eq!((d.info.pages.len(), d.info.pages[0].rotation), (2, 0));
        assert!(!d.dirty, "still saved");
        assert_eq!(d.can_redo().map(str::to_owned), redo, "the person's redo is still there");
        assert!(app.nora.changes.is_empty(), "nothing of Nora's to undo");
        app.redo();
        assert_eq!(rotation(&app, 1), 90, "and it works");
        // A single command that fails changes nothing either.
        assert!(run(&mut app, exec("page_rotate", json!({ "pages": [9], "degrees": 90 }))).is_err());
        assert_eq!(doc(&app).info.pages.len(), 2);
    }

    #[test]
    fn undo_takes_back_a_whole_batch_and_only_what_nora_did() {
        let mut app = document();
        person_rotates(&mut app, 1);
        let commands = vec![
            step("page_rotate", json!({ "pages": [1], "degrees": 90 })),
            step("doc_set_info", json!({ "key": "Title", "value": "Nora's" })),
            step("page_rotate", json!({ "pages": [1], "degrees": 90 })),
        ];
        let results = run(&mut app, DocActionKind::Batch { commands }).unwrap();
        assert_eq!(results["results"].as_array().map(Vec::len), Some(3));
        assert_eq!((rotation(&app, 0), doc(&app).info.title.as_deref()), (180, Some("Nora's")));
        assert_eq!(run(&mut app, DocActionKind::Undo).unwrap(), json!({ "undone": 3 }));
        assert_eq!((rotation(&app, 0), doc(&app).info.title.as_deref().unwrap_or_default()), (0, ""), "the whole batch is gone");
        assert_eq!(rotation(&app, 1), 90, "and the person's own change stays");
        assert_eq!(run(&mut app, DocActionKind::Undo).unwrap_err(), "Nora has nothing left to undo.");
    }

    /// The editor keeps 100 undo steps: a change of Nora's whose first steps fell out of them is not taken back in part.
    #[test]
    fn undo_takes_back_an_old_change_whole_or_not_at_all() {
        let mut app = document();
        let batch =
            |n: usize| DocActionKind::Batch { commands: (0..n).map(|_| step("page_rotate", json!({ "pages": [1], "degrees": 90 }))).collect() };
        for _ in 0..3 {
            run(&mut app, batch(40)).unwrap();
        }
        assert_eq!(doc(&app).undo_depth(), 100);
        assert_eq!(run(&mut app, DocActionKind::Undo).unwrap(), json!({ "undone": 40 }));
        assert_eq!(run(&mut app, DocActionKind::Undo).unwrap(), json!({ "undone": 40 }));
        let error = run(&mut app, DocActionKind::Undo).unwrap_err();
        assert_eq!(error, "Nora's change is too old to be taken back whole: use Undo in the editor instead.");
        assert_eq!(doc(&app).undo_depth(), 20, "none of its 20 steps left was taken back");
        assert!(app.nora.changes.is_empty());
    }

    /// Nora edits, the person undoes it and edits: the pane's Undo would take back the person's edit, so it is refused,
    /// and the person's edit stays.
    #[test]
    fn undo_is_refused_once_the_person_changed_the_document() {
        let mut app = document();
        run(&mut app, rotate(1)).unwrap();
        app.undo();
        person_rotates(&mut app, 1);
        let error = run(&mut app, DocActionKind::Undo).unwrap_err();
        assert_eq!(error, "The document has changed since Nora's last change: use Undo in the editor instead.");
        assert_eq!((rotation(&app, 0), rotation(&app, 1)), (0, 90), "the person's edit stays");
        assert!(app.nora.changes.is_empty(), "Nora's earlier changes lie under it too");
        // Nora edits again after the person: that change is hers to undo.
        run(&mut app, rotate(1)).unwrap();
        run(&mut app, DocActionKind::Undo).unwrap();
        assert_eq!((rotation(&app, 0), rotation(&app, 1)), (0, 90));
    }

    /// Reading, saving or a request that failed leave the document as Nora's last change left it: that change stays hers to
    /// undo, and the one before it too.
    #[test]
    fn undo_survives_a_reading_a_save_and_a_failed_request() {
        let mut app = document();
        let file = std::env::temp_dir().join(format!("pdfcraft-nora-undo-{}.pdf", std::process::id()));
        app.save_override = Some(file.to_string_lossy().into_owned());
        run(&mut app, rotate(1)).unwrap();
        run(&mut app, rotate(2)).unwrap();
        run(&mut app, DocActionKind::Observe).unwrap();
        assert_eq!(run(&mut app, DocActionKind::Save).unwrap()["saved"], true);
        assert!(
            run(
                &mut app,
                DocActionKind::Batch { commands: vec![step("page_rotate", json!({ "pages": [1], "degrees": 90 })), step("nope", json!({}))] }
            )
            .is_err()
        );
        assert_eq!(rotation(&app, 0), 90, "the failed batch was put back");
        run(&mut app, DocActionKind::Undo).unwrap();
        assert_eq!((rotation(&app, 0), rotation(&app, 1)), (90, 0));
        run(&mut app, DocActionKind::Undo).unwrap();
        assert_eq!(rotation(&app, 0), 0, "the change before it, in turn");
        let _ = std::fs::remove_file(file);
    }

    #[test]
    fn selecting_finds_the_nth_occurrence_and_selects_it_on_its_page() {
        let mut app = document();
        let found = run(&mut app, DocActionKind::SelectText { text: "one".into(), occurrence: 3 }).unwrap();
        assert_eq!((found["page"].clone(), found["text"].clone()), (json!(2), json!("one")));
        assert_eq!(app.views[0].selected_text().as_deref(), Some("one"), "selected as the person selects text");
        assert_eq!(app.views[0].current, 1, "on its page");
        let error = run(&mut app, DocActionKind::SelectText { text: "one".into(), occurrence: 4 }).unwrap_err();
        assert_eq!(error, "Occurrences of “one” in the document: 3 (not 4).");
        let error = run(&mut app, DocActionKind::SelectText { text: "absent".into(), occurrence: 1 }).unwrap_err();
        assert_eq!(error, "“absent” is not in the document.");
        assert_eq!(app.views[0].selected_text().as_deref(), Some("one"), "a failed search keeps the selection");
        assert!(app.nora.changes.is_empty(), "selecting is no change");
    }

    #[test]
    fn typing_goes_into_the_field_being_filled_else_a_text_box_in_free_space() {
        let mut app = open(include_bytes!("../tests/data/form.pdf").to_vec());
        // The editor of the field `city` is open; the keyboard is elsewhere (Nora would wait otherwise).
        app.views[0].forms.focus = Some(crate::forms_ui::Focus {
            name: "city".into(),
            widget: 0,
            text: String::new(),
            picked: Vec::new(),
            request_focus: false,
            select_all: false,
            calendar: None,
            calendar_rect: None,
        });
        let typed = run(&mut app, DocActionKind::TypeText { text: "Lausanne\nVaud".into(), paragraphs: true }).unwrap();
        assert_eq!((typed["field"].clone(), typed["page"].clone()), (json!("city"), json!(1)));
        let city = doc(&app).form.iter().find(|f| f.name == "city").map(|f| f.value.clone());
        assert_eq!(city, Some(vec!["Lausanne Vaud".to_string()]), "a single-line field takes the lines on one line");
        assert!(app.views[0].forms.focus.is_none(), "the editor closed on Nora's text");
        // No field being filled: a text box in free space, clear of the page's fields.
        let typed = run(&mut app, DocActionKind::TypeText { text: "Checked by Nora".into(), paragraphs: false }).unwrap();
        assert_eq!((typed["textBox"].clone(), typed["freeSpace"].clone()), (json!(true), json!(true)));
        let rect: Vec<f32> = typed["rect"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
        let d = doc(&app);
        let page = &d.info.pages[0];
        for w in d.form.iter().flat_map(|f| f.widgets.iter()) {
            let ([x0, y0], [x1, y1]) = (page.user_to_view(w.rect[0] as f32, w.rect[1] as f32), page.user_to_view(w.rect[2] as f32, w.rect[3] as f32));
            let apart = rect[2] <= x0.min(x1) || x0.max(x1) <= rect[0] || rect[3] <= y0.min(y1) || y0.max(y1) <= rect[1];
            assert!(apart, "the box {rect:?} covers a field at {:?}", [x0, y0, x1, y1]);
        }
        assert!(d.info.annotations.iter().any(|a| a.subtype == "FreeText" && a.contents.as_deref() == Some("Checked by Nora")));
    }

    #[test]
    fn a_text_box_never_covers_the_page_s_text_and_goes_at_the_top_when_nothing_is_free() {
        let mut app = document();
        // The page's lines before the box (after it, its own text is on the page too).
        let lines = line_boxes(&Pages::new(doc(&app), &app.views[0]).text(0).unwrap());
        assert_eq!(lines.len(), 2, "{lines:?}");
        let typed = run(&mut app, DocActionKind::TypeText { text: "Seen".into(), paragraphs: false }).unwrap();
        assert_eq!(typed["freeSpace"], true);
        let rect: Vec<f32> = typed["rect"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
        for line in lines {
            let apart = rect[2] <= line[0] || line[2] <= rect[0] || rect[3] <= line[1] || line[3] <= rect[1];
            assert!(apart, "the box {rect:?} covers the line {line:?}");
        }
        // A page with no room left: the top of the page, inside its margins.
        assert_eq!(free_place(&[[0.0, 0.0, 300.0, 400.0]], 300.0, 400.0, 120.0, 30.0), None);
        assert_eq!(free_place(&[], 300.0, 400.0, 120.0, 30.0), Some([MARGIN, MARGIN, MARGIN + 120.0, MARGIN + 30.0]));
        // A huge page, or one made of thousands of lines, can't hold the window: the search is bounded.
        assert_eq!(free_place(&[], 1.0e6, 1.0e6, 120.0, 30.0), Some([MARGIN, MARGIN, MARGIN + 120.0, MARGIN + 30.0]));
        let started = std::time::Instant::now();
        let place = free_place(&[[0.0, 0.0, 1.0e6, 1.0e6 - 100.0]], 1.0e6, 1.0e6, 120.0, 30.0);
        assert!(started.elapsed() < Duration::from_secs(2));
        assert!(place.is_some_and(|r| r[1] >= 1.0e6 - 100.0 + GAP), "{place:?}");
        let crowded: Vec<[f32; 4]> = (0..=MAX_TAKEN).map(|i| [0.0, i as f32, 1.0, i as f32 + 1.0]).collect();
        assert_eq!(free_place(&crowded, 600.0, 800.0, 120.0, 30.0), None);
        let full: Vec<&str> = vec!["MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"; 20];
        let mut app = open(pdf(&[&full]));
        let typed = run(&mut app, DocActionKind::TypeText { text: "Seen".into(), paragraphs: false }).unwrap();
        assert_eq!(typed["freeSpace"], false);
        assert_eq!(typed["rect"][0].as_f64().zip(typed["rect"][1].as_f64()), Some((f64::from(MARGIN), f64::from(MARGIN))));
        assert!(run(&mut app, DocActionKind::TypeText { text: "  \n ".into(), paragraphs: false }).is_err(), "nothing to type");
    }

    /// What Nora types and the text box can't draw (its font has no Cyrillic) is said to her, not drawn as `?` in silence.
    #[test]
    fn typing_says_what_the_text_box_can_t_draw() {
        let mut app = document();
        let typed = run(&mut app, DocActionKind::TypeText { text: "Привет".into(), paragraphs: false }).unwrap();
        assert!(typed["warnings"].as_array().is_some_and(|w| !w.is_empty()), "{typed}");
        let plain = run(&mut app, DocActionKind::TypeText { text: "Hello".into(), paragraphs: false }).unwrap();
        assert!(plain.get("warnings").is_none(), "{plain}");
        // Into the form field being filled, likewise.
        let mut app = open(include_bytes!("../tests/data/form.pdf").to_vec());
        app.views[0].forms.focus = Some(crate::forms_ui::Focus {
            name: "city".into(),
            widget: 0,
            text: String::new(),
            picked: Vec::new(),
            request_focus: false,
            select_all: false,
            calendar: None,
            calendar_rect: None,
        });
        let typed = run(&mut app, DocActionKind::TypeText { text: "Москва".into(), paragraphs: false }).unwrap();
        assert_eq!(typed["field"], json!("city"));
        assert!(typed["warnings"].as_array().is_some_and(|w| !w.is_empty()), "{typed}");
    }

    #[test]
    fn a_page_can_be_rendered_for_nora_to_look_at() {
        let mut app = document();
        let seen = run(&mut app, DocActionKind::RenderPage { page: 2, scale: 0.5 }).unwrap();
        assert!(seen["png"].as_str().is_some_and(|png| png.starts_with("iVBOR")), "a base64 PNG");
        assert!(seen["width"].as_u64().is_some_and(|w| w > 0 && w <= 512), "{}", seen["width"]);
        assert_eq!((seen["page"].clone(), seen["pages"].clone()), (json!(2), json!(2)));
        let error = run(&mut app, DocActionKind::RenderPage { page: 3, scale: 1.0 }).unwrap_err();
        assert_eq!(error, "There is no page 3: the last page is 2.");
    }

    #[test]
    fn save_is_the_app_s_and_says_whether_the_document_was_saved() {
        let mut app = document();
        let file = std::env::temp_dir().join(format!("pdfcraft-nora-save-{}.pdf", std::process::id()));
        app.save_override = Some(file.to_string_lossy().into_owned());
        run(&mut app, rotate(1)).unwrap();
        assert_eq!(run(&mut app, DocActionKind::Save).unwrap(), json!({ "saved": true, "name": "doc.pdf" }));
        assert!(!doc(&app).dirty);
        assert!(std::fs::read(&file).is_ok_and(|bytes| bytes.starts_with(b"%PDF")));
        let _ = std::fs::remove_file(file);
    }

    /// Nora's results carry no path of the person's files.
    #[test]
    fn what_nora_gets_back_holds_no_path_of_the_person_s_files() {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.open_bytes("doc.pdf", Some("/home/someone/private/doc.pdf".into()), pdf(&[&["Hi"]])).unwrap();
        let info = run(&mut app, exec("doc_info", json!({}))).unwrap();
        assert!(!info.to_string().contains("/home/someone"), "{info}");
        assert_eq!(without_paths(json!({ "path": [2, 1], "a": { "path": "/x" } }), 0), json!({ "path": [2, 1], "a": {} }), "a bookmark's path stays");
    }

    /// Nora's actions wait while the person drags something or types in the document, and play in their order once the
    /// person is done.
    #[test]
    fn nora_waits_while_the_person_drags_or_types_in_the_document() {
        let mut app = document();
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        let polls = start(&mut h, &outbox, &inbox, "Rotate the first page");
        let page = h.state().views[0].visible_page_rects()[0].1.center();
        h.drag_at(page);
        h.run_steps(2);
        let told = act(
            &mut h,
            &outbox,
            &inbox,
            &polls,
            json!([{ "action": "doc_execute", "command": "page_rotate", "params": { "pages": [1], "degrees": 90 }, "callback_id": "c1" }]),
        );
        assert!(!told.iter().any(|r| r.path.ends_with("receive_observation")), "not played under the person's pointer");
        assert_eq!(rotation(h.state(), 0), 0);
        assert!(h.query_by_label("Nora is waiting for you to finish what you are doing in the document.").is_some(), "the pane says why");
        h.drop_at(page);
        h.run_steps(4);
        assert_eq!(rotation(h.state(), 0), 90, "played once the pointer let go");
        assert_eq!(observation(&take(&outbox), "c1")["ok"], true);
        assert!(h.query_by_label("Nora is waiting for you to finish what you are doing in the document.").is_none());
    }

    #[test]
    fn nora_waits_while_a_form_field_has_the_keyboard() {
        let mut app = open(include_bytes!("../tests/data/form.pdf").to_vec());
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        let polls = start(&mut h, &outbox, &inbox, "Fill in the name");
        h.state_mut().views[0].forms.focus = Some(crate::forms_ui::Focus {
            name: "name".into(),
            widget: 0,
            text: "Ada".into(),
            picked: Vec::new(),
            request_focus: true,
            select_all: false,
            calendar: None,
            calendar_rect: None,
        });
        h.run_steps(3);
        let told = act(&mut h, &outbox, &inbox, &polls, json!([{ "action": "doc_type_text", "text": "Grace", "callback_id": "c1" }]));
        assert!(!told.iter().any(|r| r.path.ends_with("receive_observation")), "the person is typing: Nora waits");
        // The person leaves the field with Enter: what they typed is committed where they typed it, then Nora plays.
        h.key_press(egui::Key::Enter);
        h.run_steps(6);
        let name = doc(h.state()).form.iter().find(|f| f.name == "name").map(|f| f.value.clone());
        assert_eq!(name, Some(vec!["Ada".to_string()]), "the person's typing is in its field");
        assert_eq!(observation(&take(&outbox), "c1")["result"]["textBox"], true, "and Nora's text in a box of its own");
    }

    #[test]
    fn actions_arriving_while_others_wait_play_after_them_in_order() {
        let mut app = document();
        let ctx = egui::Context::default();
        perform(&mut app, vec![Output::Document(DocumentCall { id: 1, kind: rotate(1) })], &ctx, true);
        assert_eq!((app.nora.waiting.len(), rotation(&app, 0)), (1, 0), "waits for the person");
        let mut outputs = incoming(&mut app, false);
        outputs.push(Output::Document(DocumentCall { id: 2, kind: exec("page_rotate", json!({ "pages": [1], "degrees": 180 })) }));
        perform(&mut app, outputs, &ctx, false);
        assert_eq!(rotation(&app, 0), 270, "both, the older first");
        assert!(app.nora.waiting.is_empty());
    }

    /// The pane's words follow the interface language; her name stays hers.
    #[test]
    fn the_pane_speaks_french_when_the_interface_does() {
        let mut app = document();
        app.set_option("language", "fr").unwrap();
        app.nora_pane = true;
        app.dialog = Some(Dialog::Preferences);
        let h = window(app);
        assert!(h.query_by_label("Nora n'est pas encore disponible dans cette version").is_some());
        assert!(h.query_by_label("Nouvelle conversation").is_some());
        assert!(h.query_by_label("Demandez à Nora de lire, résumer, remplir ou annoter votre document.").is_some());
        assert!(h.query_by_label("Afficher le volet de l'assistant Nora (expérimental)").is_some(), "and its switch in Preferences");
        assert!(h.query_by_label("New conversation").is_none());
        assert!(h.query_by_label("Nora").is_some(), "her name is the same in every language");
        // What the pane answers in the person's language too.
        let mut app = document();
        run(&mut app, rotate(1)).unwrap();
        person_rotates(&mut app, 1);
        crate::i18n::set_current(crate::i18n::Lang::from_code("fr").unwrap());
        let error = run(&mut app, DocActionKind::Undo).unwrap_err();
        let typed = describe(&DocActionKind::TypeText { text: "Bonjour".into(), paragraphs: false });
        crate::i18n::set_current(crate::i18n::Lang::EN);
        assert_eq!(error, "Le document a été modifié depuis la dernière modification de Nora : utilisez plutôt la fonction Annuler de l'éditeur.");
        assert_eq!(typed, "Saisir «\u{202f}Bonjour\u{202f}»");
    }

    /// The panel's sentences are words of the interface language, not the core's English.
    #[test]
    fn actions_are_described_in_the_pane_s_words() {
        assert_eq!(describe(&DocActionKind::Save), "Save the document");
        assert_eq!(describe(&exec("page_rotate", json!({}))), "Run page_rotate");
        assert_eq!(describe(&DocActionKind::Batch { commands: vec![step("page_rotate", json!({}))] }), "Run page_rotate", "a batch of one names it");
        assert_eq!(describe(&DocActionKind::Batch { commands: vec![step("a", json!({})), step("b", json!({}))] }), "Run several commands (2)");
        let typed = describe(&DocActionKind::TypeText { text: "é".repeat(60), paragraphs: false });
        assert!(typed.starts_with("Type “") && typed.ends_with("…”"), "{typed}");
        assert_eq!((labels().describe)(&DocActionKind::Observe), "Read the document");
    }

    /// The interface draws the dash, the guillemets, the apostrophe, the curly quotes and the ellipsis of Nora's words in its
    /// Latin face: Inter comes first in every family and has them (a CJK face first in a sibling editor drew the dash as a
    /// hairline).
    #[test]
    fn the_latin_face_draws_the_punctuation_of_nora_s_words() {
        use egui::FontFamily;
        use skrifa::MetadataProvider as _;
        let all = theme::font_definitions();
        for (family, first) in [
            (FontFamily::Proportional, "Inter"),
            (FontFamily::Name("medium".into()), "Inter-Medium"),
            (FontFamily::Name("semibold".into()), "Inter-SemiBold"),
        ] {
            // egui draws a character with the first face of the family whose character map has it.
            assert_eq!(all.families[&family].first().map(String::as_str), Some(first), "{family:?}");
            let data = &all.font_data[first];
            let face = skrifa::FontRef::from_index(&data.font, data.index).unwrap();
            let missing: Vec<char> = "—«»’…“”".chars().filter(|c| face.charmap().map(*c).is_none()).collect();
            assert!(missing.is_empty(), "{first} lacks {missing:?}");
        }
    }

    /// While Nora works, the pane's Send button is a Stop button: a click ends her turn there and then, the pane says
    /// so, and the next message can go.
    #[test]
    fn stop_ends_nora_s_turn() {
        let mut app = document();
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        start(&mut h, &outbox, &inbox, "Summarize the document");
        assert!(h.state().nora.session.is_busy(), "Nora is at work");
        h.get_by_label("Stop").click();
        h.run_steps(4);
        assert!(!h.state().nora.session.is_busy(), "her turn ended");
        assert!(h.query_by_label("Stopped.").is_some(), "the pane says Nora was stopped");
        assert!(h.query_by_label("Send").is_some(), "the next message can go");
    }

    /// A call that got no answer at all is said in the interface language, not with the browser's English reason.
    #[test]
    fn no_answer_from_nora_is_said_in_the_interface_language() {
        let mut app = document();
        app.set_option("language", "fr").unwrap();
        let (outbox, inbox) = connect(&mut app);
        let mut h = window(app);
        h.state_mut().nora.panel.set_draft("Bonjour");
        h.run_steps(2);
        h.get_by_label("Envoyer").click();
        h.run_steps(4);
        let sent = take(&outbox);
        let body = json!({ "exception": "No answer from Nora: TypeError: Failed to fetch" }).to_string();
        inbox.lock().unwrap().push(HttpResponse { id: sent[0].id, status: 0, body });
        h.run_steps(4);
        assert!(h.query_by_label("Nora n'a pas pu être jointe. Vérifiez la connexion et réessayez.").is_some());
        assert!(h.query_by_label("No answer from Nora: TypeError: Failed to fetch").is_none());
    }
}
