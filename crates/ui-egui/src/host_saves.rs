// //// Neoffice — added file (no upstream equivalent): what the page that frames the editor answers to a save (`pdfcraft:saved`).

//! The answers of the page that frames PdfCraft (`?host=parent`) to the files it was handed.
//!
//! Upstream's bridge has no answer to a save. [`PdfCraftApp::save_then`] posts `pdfcraft:save`
//! and calls `mark_saved` at once, so the document counts as saved even when the page could not
//! store the file (no right on the folder, a lock, the network). For an editor that replaces a
//! server-side one, that is work lost without a word. Neoffice adds `pdfcraft:saved { name, ok,
//! error }`, which the page sends once it has tried to store `name`:
//!
//! - `ok: false`: the web shell queues it in [`RefusedSaves`]. The next frame puts the document
//!   back to unsaved ([`pdfcraft_engine::Session::mark_unsaved`]) and the notice says why. The
//!   unsaved-changes flag and the `pdfcraft:dirty` report follow by themselves, as both are read
//!   from that state every frame.
//! - `ok: true`, no `ok`, or no answer at all: nothing changes, as upstream's bridge behaves.
//!
//! The web shell (`apps/pdfcraft-web`) owns the browser side and only reads the message's fields.
//! What they mean, and the part that touches the session, live here in the interface crate so
//! that they are tested without a browser.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use pdfcraft_engine::DocId;

use crate::PdfCraftApp;

/// The longest file name a notice shows (as in the other Craft editors' bridges): the page is
/// trusted with the file, not with the size of the notice.
const MAX_NAME_CHARS: usize = 120;
/// The longest reason a notice shows.
const MAX_REASON_CHARS: usize = 300;

/// What the notice says when the page gave no reason, worded as in the other Craft editors'
/// bridges. It is not translated: it fills the reason's place like an engine or system message.
const NO_REASON: &str = "the page did not say why";

/// The saves the embedding page could not store, as `(name, reason)`: a queue shared between the
/// web shell, which fills it from the browser's `message` events, and the app, which empties it
/// every frame (like [`crate::Inbox`] and [`crate::FailedInbox`]). `name` is the one the page was
/// given with the file, the document's name; an empty `reason` means the page gave none.
#[derive(Clone, Default)]
pub struct RefusedSaves(Arc<Mutex<Vec<(String, String)>>>);

impl RefusedSaves {
    /// Take in one `pdfcraft:saved { name, ok, error }` from the page. Only `ok: false` is a
    /// refusal; `true`, a missing or a mistyped `ok` leave the document saved, as it was before
    /// this answer existed. Returns whether the answer was queued, so the shell knows to ask for
    /// a frame.
    pub fn answer(&self, name: Option<String>, ok: Option<bool>, error: Option<String>) -> bool {
        if ok != Some(false) {
            return false;
        }
        self.queue().push((name.unwrap_or_default(), error.unwrap_or_default()));
        true
    }

    /// Whether no refusal waits for the next frame.
    pub fn is_empty(&self) -> bool {
        self.queue().is_empty()
    }

    fn take(&self) -> Vec<(String, String)> {
        std::mem::take(&mut *self.queue())
    }

    /// A refusal queued by code that panicked is still a refusal: a poisoned lock is read through.
    fn queue(&self) -> MutexGuard<'_, Vec<(String, String)>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl PdfCraftApp {
    /// Apply what the embedding page said about the saves it was handed since the last frame.
    ///
    /// An answer names a file, not a save. When the same name was saved twice before the page
    /// answered, the answer is taken for the last save: the document is marked unsaved whichever
    /// of the two failed, which costs at most one more Save. When no open document carries the
    /// name (no name at all, a document closed right after its save, a page that renamed the
    /// file), the answer is taken for the active document, the safe side of a doubt. The notice is
    /// shown in every case.
    pub(crate) fn settle_refused_saves(&mut self) {
        for (name, reason) in self.refused_saves.take() {
            let (name, documents) = self.documents_named(&name);
            for id in documents {
                self.session.mark_unsaved(id);
            }
            let reason = reason.trim();
            let reason = if reason.is_empty() { NO_REASON } else { reason };
            self.notify_fmt("Couldn't save {name}: {e}", &[("name", &tidy(&name, MAX_NAME_CHARS)), ("e", &tidy(reason, MAX_REASON_CHARS))]);
        }
    }

    /// The open documents an answer about `name` is for, with the name to show.
    fn documents_named(&self, name: &str) -> (String, Vec<DocId>) {
        let named: Vec<DocId> = self.session.docs().iter().filter(|doc| doc.name == name).map(|doc| doc.id).collect();
        if !named.is_empty() {
            return (name.to_owned(), named);
        }
        let shown = if name.trim().is_empty() { None } else { Some(name.to_owned()) };
        match self.active_ids().and_then(|(_, id)| self.session.get(id)) {
            Some(doc) => (shown.unwrap_or_else(|| doc.name.clone()), vec![doc.id]),
            None => (shown.unwrap_or_default(), Vec::new()),
        }
    }
}

/// `text` on one line and at most `max` characters, for a notice that has room for neither.
fn tidy(text: &str, max: usize) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.chars().count() <= max {
        return line;
    }
    let mut cut: String = line.chars().take(max.saturating_sub(1)).collect();
    cut.push('…');
    cut
}
