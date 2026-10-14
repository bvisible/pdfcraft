# Neoffice fork markers

Map of **our** divergence from upstream (`storytold/pdfcraft`) in this fork (`bvisible/pdfcraft`, branch
`version-15`). Everything that can carry a comment carries a `//// Neoffice — …` marker in place:
`// //// Neoffice — …` in Rust (a bare `////` is an error for clippy), `# //// Neoffice — …` in TOML
and YAML, `<!-- //// Neoffice — … -->` in Markdown. This file holds the base of the divergence and
what cannot carry a comment.

At the next upstream merge, `grep -rn "////"` in the source plus this file give the complete
picture of what is ours and why.

- **Upstream**: `storytold/pdfcraft`, branch `main`.
- **Base of the divergence**: `7163ba3dee8becfff6871c9a65d0d8314314742b`, the tag `v0.7.1`, where `version-15` was cut on 2026-10-13.
  Move it at each upstream merge. The check works it out itself (the common ancestor with
  upstream's `main`), so this line is for the reader.

## What the fork adds

Three workflow files and an answer to a save in the browser build's host bridge. Every new file
carries its own `//// Neoffice — added file` header; the few changes to upstream's files carry a
marker in place.

- `.github/workflows/neoffice-web.yml`: builds the browser version on a GitHub runner, started by
  hand, with the packaging script upstream's own release job runs. Upstream's workflows are
  disabled on this fork, because they need upstream's signing secrets.
- `.github/workflows/neoffice-markers.yml`: fails a push to `version-15` when a change to
  upstream's code since the common ancestor carries no marker.
- `.github/workflows/neoffice-ci.yml`: runs upstream's whole quality gate, `cargo xtask ci`, in one
  job on a GitHub runner (by hand, or on a push that touches code), because the maintainer's Mac has
  too little disk to run it for every change.
- **The answer to a save, in the host bridge of the browser build** (`?host=parent`: the
  `postMessage` contract of a same-origin page that frames the editor, `pdfcraft:ready`, `open`,
  `save`, `dirty` and `failed`, which WordCraft, GridCraft and DeckCraft speak too, so that one host
  page drives every Craft editor). Ours is `pdfcraft:saved { name, ok, error }`, which the page sends
  once it tried to store a file that `pdfcraft:save` gave it. Upstream's bridge has no answer: the
  document counts as saved as soon as the file is handed over, even when the page could not store it
  (a right refused, a lock, the network). `ok: false` puts the document back to unsaved (the
  unsaved-changes flag and `pdfcraft:dirty` follow) and the notice says why, with the existing
  `Couldn't save {name}: {e}` string; `ok: true` or no answer changes nothing. An answer names a file,
  not a save: when the same name was saved twice before the answer, it counts for the last save, and
  an answer that no open document carries (no name, a document closed right after its save) is taken
  for the active document, the safe side of a doubt.
  New files: `crates/ui-egui/src/host_saves.rs` (the queue the web shell fills and the app empties
  every frame, and what an answer does to the session) and `crates/ui-egui/tests/host_saves.rs`.
  Marked changes: `crates/ui-egui/src/lib.rs` (the module, the queue as a field, and one call in
  `logic`), `crates/engine/src/lib.rs` and `crates/engine/src/tests.rs` (`Session::mark_unsaved` and
  its test), `apps/pdfcraft-web/src/main.rs` (reads the message). Drop it when upstream ships its own
  answer to a save, keeping `pdfcraft:saved`.
- **`?author=` always wins** (`apps/pdfcraft-web/src/main.rs`). Upstream used it only while the
  comment author was still the web default, "Guest". The browser saves that name with the other
  settings, so the first name a host passed stuck: the next user of the same browser, or the same
  user renamed, kept signing comments as the first. The host knows who is signed in; WordCraft's
  bridge does the same.

- **What a hosting page hides** (`PdfCraftApp::hide_community`, set by `apps/pdfcraft-web/src/main.rs` next to the host
  bridge, so only when a page frames the editor): upstream's community links are not shown (the title bar's Discord
  button, the Home page's community card), and the link commands (`help.discord`…) open nothing. Marked changes in
  `crates/ui-egui/src/lib.rs` (the field), `chrome.rs`, `home.rs` and `commands.rs`; new test file
  `crates/ui-egui/tests/host_community.rs`. Every other build behaves as before.

## Files that cannot carry a comment

None so far.
