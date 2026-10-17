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

Three workflow files, an answer to a save in the browser build's host bridge, and the features below.
Every new file carries its own `//// Neoffice — added file` header; the changes to upstream's files
carry a marker in place.

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

- **A hosting page opens the documents** (`PdfCraftApp::host_owns_documents`, set by `apps/pdfcraft-web/src/main.rs`
  next to the host bridge, so only when a page frames the editor): the editor offers no Open, no Recent list and takes
  no document dropped on it, and `file.open`, Open Recent and the other ways of opening a file say that the page opens
  the documents. The Combine files tab and every tool are as before. Marked changes in `crates/ui-egui/src/lib.rs`
  (the field, `host_refuses_open` and its four callers) and `home.rs`; new test file
  `crates/ui-egui/tests/host_documents.rs`. Every other build behaves as before.

- **The account button of the tab strip** (new file `crates/ui-egui/src/account.rs`; one marked line in `lib.rs` and two
  in `chrome.rs`'s `tab_strip`; new test file `crates/ui-egui/tests/account_button.rs`): at the right end of the strip,
  the window's top row, the button WordCraft has at the end of its title bar, so the four editors show the person the
  same way: a round badge in the accent colour, a square of the strip's control height, with up to two initials (the
  first letters of the words of `CommentPrefs::author`, the name a hosting page passes as `?author=`), and the name,
  nothing else, as the hover text. The initials take, of white and the theme's dark, the one that reads better on the
  accent, at least 4.5:1 (a test checks it): white on the light theme (5.4:1, the text of the primary buttons there),
  the strip's own colour on the dark theme (5.2:1; white reads at 3.3:1 there). The room the strip keeps
  for its controls on the right counts the button, so the tabs never run under it. It shows only when a name is known:
  not while the name is blank or PdfCraft's default for a user nobody named (the login name, "Guest" on the web). No
  new text.

- **The Nora assistant pane** (new files `crates/ui-egui/src/nora_pane.rs`, with its tests, `crates/ui-egui/src/nora_origin.rs`
  and `apps/pdfcraft-web/src/nora.rs`; marked changes in the workspace `Cargo.toml`, `crates/ui-egui/Cargo.toml` and
  `apps/pdfcraft-web/Cargo.toml`, `deny.toml` (the fork's git source allowed), `crates/ui-egui/src/lib.rs` (the modules, `NoraHttpFn`, `NoraInbox`, the fields
  `nora_pane`, `nora_http`, `nora_inbox` and `nora`, the setting kept in `persist`/`restore`, `set_option("nora")`, the pane
  in `ui`), `commands.rs` (`ui.nora`), `js_ui.rs` (Preferences ▸ Assistant), `apps/pdfcraft-web/src/main.rs`, and in three
  crates below the shell: `crates/engine/src/commands.rs` (the registry command `ui.nora`, "Nora assistant", in the palette
  and out of the menus), `crates/engine/src/lib.rs` (`Session::checkpoint`, `Session::restore`, `Checkpoint`, tested in
  `tests.rs`; `Document::undo_depth`), `crates/automation/src/lib.rs` (`Automation::session_mut`; `text_lines` refused,
  as `text_extract` is, in a document that forbids copying its text, tested in `crates/automation/tests/automation.rs`),
  `crates/forms/src/af.rs` (`calculate_js`: a `*/` in a calculation's notation can no longer end its script comment and
  plant a script, tested there) and `xtask/src/layers.rs` (the edge `ui-egui → automation` allowed, with its test)):
  Neoffice's assistant, the chat at the right of the window that NeoWrite
  (WordCraft) has, so that the same Nora works in every editor. It is built on `craft-nora` and `craft-nora-ui` from
  Neoffice's craft-libs fork, at the revision WordCraft uses; to follow a newer one, move the revision in `Cargo.toml`.
  - **When it shows.** Off by default. Preferences ▸ Assistant, `ui.nora` (a toggle, in the command palette as "Nora
    assistant") or `--nora on|off` (`set_option`) shows it, and the setting is kept (`nora_pane`). The browser build opens
    it when the page asks for Nora (`?nora`, read by name with `URLSearchParams`) or makes Nora's calls itself
    (`window.pdfcraftNoraCall(path, body)` on the page or its same-origin parent, as Neodesk does). Outermost at the
    right, not in full screen.
  - **Where the calls go** (`nora_origin.rs`, tested natively; the rule is common to Neoffice's editors): `?nora` is the
    page's own origin; `?nora=<origin>` is taken only when it is the page's own origin (case, a trailing slash and the
    default port make no difference) or when the page and that origin are both loopback (`127.0.0.1`, `localhost`,
    `[::1]`, the workshop's stand-in server); any other origin the address names is ignored, the calls go to the page's
    own origin and the console says why. Another origin only from `window.pdfcraftNora.origin`, set by the hosting page.
    The page's credentials (a Frappe page's CSRF token, `window.pdfcraftNora.headers`) go only with calls to its own
    origin.
  - **The connection is the host's**: the pane never opens one (`nora_http`, `nora_inbox`); without one it says that Nora
    isn't available and offers no text box.
  - **Nora's actions are PdfCraft's commands, played by the headless tools of `pdfcraft-automation`** (the surface of the
    MCP server and `pdfcraft-cli run`; the window's session is swapped into the tools for each call, which is why the
    egui shell now depends on the automation crate, the way upstream meant its control channel to use the tool table).
    Execute and Batch: a tool by its name (`page_rotate`, `comment_add`) or a registry command by its id (`page.rotate`),
    played by the one tool that automates it; a registry command with no tool works only through the interface and is
    refused, as is one several tools play (the error names them). Refused (`REFUSED` in `nora_pane.rs`, beyond `file.`
    and `app.`): the interface (`ui.`, `view.`, `help.`, panels, hidden comments, reading colours, the date format), the
    person's undo history (`edit.undo`, `edit.redo`, `edit_undo`, `edit_redo`), their clipboard (`edit.snapshot`,
    `page.copy`, `page.cut`, `page.paste`), other documents and files (open, create, combine, split, extract, compare,
    close, save, export, import, print), links (`link_open`), scripts and stored automation (`js_`, `tools.`,
    `form_set_script`, `form_set_actions`, `actions.`, `action_`, `command_run`, `command_batch`) and signing (`sign.`
    and the digital ID tools, and Fill & Sign's typed signature and initials, `REFUSED_VALUES`); any command with a
    parameter that names a file (`paths`, `file`, `files`, `filename`, `folder`, `directory`, `out`, `out_dir`, `output`…,
    and `path` and `image` when they hold text: a bookmark's `path` is a list of numbers) or a password (a name holding
    `password` or `passphrase`), at any depth and in any case; and, in a document that forbids copying its text, every
    tool that gives it (`COPYING_TOOLS`: its lines, paragraphs and objects, a search, OCR, the accessibility tags, what a
    redaction covers), SelectText and RenderPage. `doc` is always the active document. A batch holds
    50 commands at most. Results carry no path of the person's files. Observe: the text of the pages, the visible ones
    first, each under `Page n (current, visible):`, cut at 50,000 characters and 50 pages, the page count, the current
    page and the selection (no text of a document that forbids copying it). SelectText: the n-th occurrence, pages in
    order, as written, selected as the person selects text, its page brought into view. TypeText: into the form field
    whose editor is open (not while it has the keyboard: the person is typing there), else into a text box comment on the
    current page, in the first free place (never over the page's text, comments or fields), else at the top of the page,
    signed with the person's name; what it can't draw (`?`) is said in `warnings`. RenderPage: `render_preview`, 1,024 pixels on the longer side at scale 1. Save: the
    app's own Save. Nora's actions wait, in their order, while the person drags, resizes or holds a button, has a dialog,
    a prompt or the palette open, types in a text box that isn't the pane's, or types, draws or chooses in the document;
    the pane says so.
  - **A failed request leaves the document as it was**, its undo and redo history and its unsaved flag included
    (`Session::checkpoint` before, `Session::restore` on failure). **Undo** (the pane's button) takes back Nora's last
    change whole, by the undo steps it took, and only while the document is at the generation that change left it at; a
    reading, a save or a request put back keep it so, and once one change is undone the one before it, if nothing came
    between them, and only when the editor still keeps all its undo steps (`Document::undo_depth`). Otherwise it is
    refused and Nora's earlier changes are forgotten. An action that changed nothing
    answers `"changed": false`.
  - **The words** go through the interface's catalogs: every new string is in all 21 of them (translated by a model,
    French reusing the wording of WordCraft's pane). "Nora" is a name and stays untranslated.

## Files that cannot carry a comment

| File | Why it is ours |
|---|---|
| `Cargo.lock` | `rustls` is at 0.23.46 (upstream's file has 0.23.45): `cargo deny` refuses 0.23.45, RUSTSEC-2026-0349 (a ServerHello that downgrades later connections to classical key exchange), and the quality gate stops there. On a merge conflict, take upstream's file; if it still has `rustls` below 0.23.46, run `cargo update -p rustls`. It also holds the Nora pane's dependencies: `craft-nora` and `craft-nora-ui` from `bvisible/craft-libs` (git, revision `8fee6abfbd33ae783a4d74bbcf005ae3ca986f84`, the one WordCraft uses), and the new edges of `pdfcraft-ui-egui` (to them and to `pdfcraft-automation`) and of `pdfcraft-web` (to `serde_json`). At an upstream merge, take upstream's lock and let cargo add these back. |
| `crates/ui-egui/src/i18n/ar.tsv`, `crates/ui-egui/src/i18n/bg.tsv`, `crates/ui-egui/src/i18n/cs.tsv`, `crates/ui-egui/src/i18n/de.tsv`, `crates/ui-egui/src/i18n/el.tsv`, `crates/ui-egui/src/i18n/es.tsv`, `crates/ui-egui/src/i18n/fr.tsv`, `crates/ui-egui/src/i18n/hu.tsv`, `crates/ui-egui/src/i18n/it.tsv`, `crates/ui-egui/src/i18n/ja.tsv`, `crates/ui-egui/src/i18n/kk.tsv`, `crates/ui-egui/src/i18n/lt.tsv`, `crates/ui-egui/src/i18n/pl.tsv`, `crates/ui-egui/src/i18n/pt-br.tsv`, `crates/ui-egui/src/i18n/ru.tsv`, `crates/ui-egui/src/i18n/sl.tsv`, `crates/ui-egui/src/i18n/te.tsv`, `crates/ui-egui/src/i18n/tr.tsv`, `crates/ui-egui/src/i18n/uk.tsv`, `crates/ui-egui/src/i18n/zh-hans.tsv`, `crates/ui-egui/src/i18n/zh-hant.tsv` | The words of the Nora pane, its switch in Preferences and its command, appended to each catalog under a `# //// Neoffice` comment line (the catalogs take comments, but the marker check counts `.tsv` files as having none). At an upstream merge, keep both sides: upstream's entries, then ours. |
