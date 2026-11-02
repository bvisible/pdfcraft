// //// Neoffice — added file (no upstream equivalent)
// Why: two things the Neoffice front ends need of the session and upstream has none of. The browser build hands a saved file to the
// page that frames the editor and calls `mark_saved` at once, so when that page then says it could not store the file the document
// must go back to unsaved (`mark_unsaved`). And the Nora assistant pane (ui-egui's `nora_pane.rs`) plays several commands for one
// request, so a request that fails half-way must leave the document as it was (`checkpoint`, `restore`). They sit in a file of
// their own, not in `save.rs` and `editing.rs`, so that an upstream merge never has to carry them.

use std::sync::Arc;

use super::Session;
use crate::history::Editor;
use crate::{DocId, EditError, guard, js};

/// A document's editing state at one moment, given back by [`Session::restore`].
pub struct Checkpoint {
    id: DocId,
    editor: Option<Editor>,
    dirty: bool,
    xfa_template: Option<Arc<pdfcraft_xfa::model::Template>>,
    xfa_scripts_off: bool,
    xfa_warnings: Vec<String>,
    js_output: js::JsOutput,
}

impl Checkpoint {
    /// The document it belongs to.
    pub fn doc(&self) -> DocId {
        self.id
    }
}

impl Session {
    /// Put document `id` back to unsaved after [`Self::mark_saved`] recorded a save that did not
    /// reach its destination. Nothing else changes: the working file, the undo and redo history
    /// and the `generation` stay as they are (the next save writes the same bytes again, with
    /// any later edit appended). Does nothing when `id` is not open.
    pub fn mark_unsaved(&mut self, id: DocId) {
        if let Some(doc) = self.docs.iter_mut().find(|d| d.id == id) {
            doc.dirty = true;
            // It stays so whatever Undo or Redo bring back (`sync_dirty` reads `unsaved_file` since upstream 0.11): the file
            // `mark_saved` took as saved never reached its destination.
            if let Some(editor) = doc.editor.as_mut() {
                editor.unsaved_file = true;
            }
        }
    }

    /// The editing state of document `id` now, to put back with [`Self::restore`]: its object graph, its undo and redo
    /// history, its passwords and whether it has unsaved changes. Cheap: the object graph and the history share their
    /// data with the document. `None` when `id` is not open.
    ///
    /// A request that fails half-way must leave the document as it was, its undo and redo history and its unsaved flag
    /// included: undoing the steps that ran would leave them on the redo list and the document marked unsaved. Upstream's
    /// tools stop at the first error and keep what ran.
    pub fn checkpoint(&self, id: DocId) -> Option<Checkpoint> {
        let doc = self.get(id)?;
        Some(Checkpoint {
            id,
            editor: doc.editor.clone(),
            dirty: doc.dirty,
            xfa_template: doc.xfa_template.clone(),
            xfa_scripts_off: doc.xfa_scripts_off,
            xfa_warnings: doc.xfa_warnings.clone(),
            js_output: doc.js_output.clone(),
        })
    }

    /// Put the document of `checkpoint` back as it was then, and rebuild what is displayed from it. The `generation`
    /// moves on, as for any change of the working file, so whatever was drawn from the state given up is drawn again.
    pub fn restore(&mut self, checkpoint: Checkpoint) -> Result<(), EditError> {
        let doc = self.doc_mut(checkpoint.id)?;
        doc.edit_notices.clear();
        doc.editor = checkpoint.editor;
        doc.dirty = checkpoint.dirty;
        doc.xfa_template = checkpoint.xfa_template;
        doc.xfa_scripts_off = checkpoint.xfa_scripts_off;
        doc.xfa_warnings = checkpoint.xfa_warnings;
        doc.js_output = checkpoint.js_output;
        Self::adopt_keys(doc);
        doc.generation += 1;
        let refreshed = guard(|| Self::refresh(doc)).unwrap_or_else(|m| Err(EditError::Reopen(m)));
        refreshed.inspect_err(|_| Self::show_released(doc))
    }
}
