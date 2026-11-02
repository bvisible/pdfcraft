// //// Neoffice — added file (no upstream equivalent)
// Why: a page that frames the editor (the Drive of the Suite, Neodesk's window) gives it one document to edit, and what
// the editor might open or close beside it is refused or not offered. These are the questions the guards ask
// (`host_owns_documents`, `hide_community`): they sit in a file of their own so that an upstream merge, which moves the
// editor's code about, never has to carry them. The guards themselves are marked where they stand.

use crate::PdfCraftApp;

impl PdfCraftApp {
    // A page that opens the documents (`host_owns_documents`) has the one to edit; true, and the person is told, when an
    // open has to be refused.
    pub(crate) fn host_refuses_open(&mut self) -> bool {
        if !self.host_owns_documents {
            return false;
        }
        self.notify("This document is opened from the page that hosts the editor.".to_string());
        true
    }

    // The commands that open or create another document. A page that opens the documents (`host_owns_documents`) neither
    // offers them (menus, palette, tab strip) nor runs them: its one document is the page's.
    const HOST_HIDDEN_COMMANDS: &'static [&'static str] = &[
        "file.open",
        "file.open_recent",
        "file.clear_recent",
        "file.pin_folder",
        "create.blank",
        "create.file",
        "create.multiple",
        "create.multiple_separate",
        "create.images",
        "create.clipboard",
        "page.combine",
        // What only makes another document from the page's one (a comment summary) or opens another to compare with.
        "comment.summarize",
        "doc.compare",
        // The page's document stays in its tab: closing it would leave the editor with nothing to show.
        "file.close",
        "file.close_all",
    ];

    pub(crate) fn host_hides(&self, id: &str) -> bool {
        (self.host_owns_documents && Self::HOST_HIDDEN_COMMANDS.contains(&id))
            // Upstream's links too (the Help menu's Discord, web page, GitHub and website), and its update check, where the
            // page hides the community's (`hide_community`): in the browser build the update check opens upstream's
            // releases page, and the hosting page serves the editor's version.
            || (self.hide_community && (id == "help.check_updates" || pdfcraft_engine::links::for_command(id).is_some()))
    }

    // Told, not offered. What a shortcut or the control channel reaches among the hidden commands gets the page's own
    // words: opened for the commands that bring another document, closed for the ones that close.
    pub(crate) fn host_refuses_command(&mut self, id: &str) {
        if id == "help.check_updates" || pdfcraft_engine::links::for_command(id).is_some() {
            // Hidden with upstream's links: nothing to say, nothing to open.
        } else if matches!(id, "file.close" | "file.close_all") {
            self.notify("This document is closed from the page that hosts the editor.".to_string());
        } else {
            self.host_refuses_open();
        }
    }

    // "Create a PDF", "Combine files" and "Compare files" only open or create other documents; a page that opens the
    // documents has none of them among its tools.
    pub(crate) fn host_hides_tool(&self, id: &str) -> bool {
        self.host_owns_documents && matches!(id, "create" | "combine" | "compare")
    }
}
