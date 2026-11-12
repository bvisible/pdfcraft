// //// Neoffice — added file (no upstream equivalent)
//! Catalan words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora encara no està disponible en aquesta versió"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora espera que s'acabi el que es fa al document."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Responeu aquesta targeta al xat de Nora dins de Neoffice: NeoPDF encara no la pot gestionar.",
    ),
    ("There is no document open for Nora to work in.", "No hi ha cap document obert on Nora pugui treballar."),
    ("`{command}` is refused: Nora may not run it.", "`{command}` està refusada: Nora no pot executar-la."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "`{command}` no la pot executar Nora a NeoPDF: només funciona a través de la interfície.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "`{command}` la proposen diverses eines ({tools}): indica quina cal executar.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` no és una ordre de NeoPDF: command_list enumera les que Nora pot executar.",
    ),
    (
        "`{command}` is refused: its parameters are nested too deeply.",
        "`{command}` està refusada: els seus paràmetres estan imbricats massa profundament.",
    ),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "`{command}` està refusada: `{key}` arribaria a un fitxer de l'ordinador.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "`{command}` està refusada: `{key}` és una contrasenya, que només s'introdueix a mà.",
    ),
    ("The parameters of `{command}` must be an object.", "Els paràmetres de `{command}` han de ser un objecte."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora treballa al document obert en aquesta finestra: `doc` designa un altre document.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "`{command}` ha fallat de manera inesperada ({error}); el document no s'ha modificat.",
    ),
    ("Commands in one batch: {count} at most.", "Ordres en un mateix lot: {count} com a màxim."),
    ("`{command}` failed: {error}", "`{command}` ha fallat: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Ocurrències de «{text}» al document: {count} (no {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "«{text}» no es troba a les primeres {pages} pàgines del document."),
    ("“{text}” is not in the document.", "«{text}» no es troba al document."),
    ("There is no text to type.", "No hi ha cap text per escriure."),
    ("There is no page {page}: the last page is {count}.", "No hi ha la pàgina {page}: l'última és la pàgina {count}."),
    ("{name} was not saved: {why}", "No s'ha pogut desar {name}: {why}"),
    ("Nora has nothing left to undo.", "Nora no té res més per desfer."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "El document ha canviat des de l'última modificació de Nora: cal usar en lloc seu la funció Desfés de l'editor.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Desfer ha fallat al pas {step} de {count}: {error}"),
    ("New conversation", "Nova conversa"),
    ("Undo the last change Nora made to the document", "Desfés l'última modificació que Nora ha fet al document"),
    ("Ask Nora…", "Demana a Nora…"),
    ("Send", "Envia"),
    ("Nora is working…", "Nora està treballant…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Demana a Nora que llegeixi, resumeixi, ompli o anoti el teu document."),
    ("Nora wants to:", "Nora vol:"),
    ("Refuse", "Refusa"),
    ("Read the document", "Llegeix el document"),
    ("Run {command}", "Executa {command}"),
    ("Run several commands ({count})", "Executa diverses ordres ({count})"),
    ("Select “{text}”", "Selecciona «{text}»"),
    ("Type “{text}”", "Escriu «{text}»"),
    ("Look at page {page}", "Consulta la pàgina {page}"),
    ("Save the document", "Desa el document"),
    ("Undo the last change", "Desfés l'última modificació"),
    ("Assistant", "Assistent"),
    ("Show the Nora assistant pane (experimental)", "Mostra el panell de l'assistent Nora (experimental)"),
    ("Nora assistant", "Assistent Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "`{command}` està refusada: la signatura i les inicials les afegeix només la persona usuària.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` està refusada: la protecció d'aquest document prohibeix copiar-ne el text.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "La modificació de Nora és massa antiga per desfer-la completa: cal usar en lloc seu la funció Desfés de l'editor.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Atura"),
    ("Stopped.", "Aturat."),
    ("Nora did not answer in time.", "Nora no ha respost a temps."),
    ("Nora could not be reached. Check the connection and try again.", "No s'ha pogut contactar amb Nora. Comprova la connexió i torna-ho a provar."),
    ("Your session has expired. Sign in again.", "La sessió ha caducat. Torna a iniciar la sessió."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "El servidor no ha pogut respondre (error {status}). Torna-ho a provar d'aquí a una estona.",
    ),
    ("Nora's answer could not be read.", "No s'ha pogut llegir la resposta de Nora."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    (
        "This tab was in the background: Nora could not act in the document.",
        "Aquesta pestanya era en segon pla: Nora no ha pogut fer res al document.",
    ),
    ("Resume", "Reprèn"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "No s'ha fet res al document: Nora no ha pogut actuar-hi a temps. Torna a preguntar si encara ho necessites.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "No s'ha fet tot: una part no s'ha pogut executar al document a temps.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Demanar a Nora"),
    ("Nora, the Neoffice assistant", "Nora, l'assistenta de Neoffice"),
];
