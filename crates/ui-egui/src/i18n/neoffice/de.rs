// //// Neoffice — added file (no upstream equivalent)
//! German words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora ist in dieser Version noch nicht verfügbar"),
    (
        "Nora is waiting for you to finish what you are doing in the document.",
        "Nora wartet, bis du fertig bist mit dem, was du gerade im Dokument machst.",
    ),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Beantworten Sie diese Karte im Chat von Nora in Neoffice: NeoPDF kann die Antwort noch nicht verarbeiten.",
    ),
    ("There is no document open for Nora to work in.", "Kein Dokument ist geöffnet, in dem Nora arbeiten kann."),
    ("`{command}` is refused: Nora may not run it.", "Der Befehl `{command}` wird abgelehnt: Nora darf ihn nicht ausführen."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "Der Befehl `{command}` kann von Nora in NeoPDF nicht ausgeführt werden: Er funktioniert nur über die Oberfläche.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "Der Befehl `{command}` wird von mehreren Werkzeugen angeboten ({tools}): Nenne das Werkzeug, das ausgeführt werden soll.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` ist kein Befehl von NeoPDF: command_list nennt die Befehle, die Nora ausführen kann.",
    ),
    (
        "`{command}` is refused: its parameters are nested too deeply.",
        "Der Befehl `{command}` wird abgelehnt: Seine Parameter sind zu tief verschachtelt.",
    ),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "Der Befehl `{command}` wird abgelehnt: `{key}` würde auf eine Datei des Computers zugreifen.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "Der Befehl `{command}` wird abgelehnt: `{key}` ist ein Passwort, das nur du eingibst.",
    ),
    ("The parameters of `{command}` must be an object.", "Die Parameter von `{command}` müssen ein Objekt sein."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora arbeitet im Dokument, das in diesem Fenster geöffnet ist: `doc` nennt ein anderes Dokument.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "Der Befehl `{command}` ist unerwartet fehlgeschlagen ({error}); das Dokument wurde nicht verändert.",
    ),
    ("Commands in one batch: {count} at most.", "Befehle in einem Stapel: höchstens {count}."),
    ("`{command}` failed: {error}", "Der Befehl `{command}` ist fehlgeschlagen: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Vorkommen von „{text}“ im Dokument: {count} (nicht {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "„{text}“ kommt auf den ersten {pages} Seiten des Dokuments nicht vor."),
    ("“{text}” is not in the document.", "„{text}“ kommt im Dokument nicht vor."),
    ("There is no text to type.", "Es gibt keinen Text zum Eingeben."),
    ("There is no page {page}: the last page is {count}.", "Es gibt keine Seite {page}: die letzte ist Seite {count}."),
    ("{name} was not saved: {why}", "Die Datei {name} wurde nicht gespeichert: {why}"),
    ("Nora has nothing left to undo.", "Nora hat nichts mehr rückgängig zu machen."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Das Dokument wurde seit Noras letzter Änderung verändert: Verwende stattdessen Rückgängig im Editor.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Rückgängig machen ist bei Schritt {step} von {count} fehlgeschlagen: {error}"),
    ("New conversation", "Neues Gespräch"),
    ("Undo the last change Nora made to the document", "Die letzte Änderung rückgängig machen, die Nora am Dokument vorgenommen hat"),
    ("Ask Nora…", "Frag Nora…"),
    ("Send", "Senden"),
    ("Nora is working…", "Nora arbeitet…"),
    (
        "Ask Nora to read, summarize, fill in or annotate your document.",
        "Bitte Nora, dein Dokument zu lesen, zusammenzufassen, auszufüllen oder mit Anmerkungen zu versehen.",
    ),
    ("Nora wants to:", "Nora möchte:"),
    ("Refuse", "Ablehnen"),
    ("Read the document", "Dokument lesen"),
    ("Run {command}", "{command} ausführen"),
    ("Run several commands ({count})", "Mehrere Befehle ausführen ({count})"),
    ("Select “{text}”", "„{text}“ auswählen"),
    ("Type “{text}”", "„{text}“ eingeben"),
    ("Look at page {page}", "Seite {page} ansehen"),
    ("Save the document", "Dokument speichern"),
    ("Undo the last change", "Letzte Änderung rückgängig machen"),
    ("Assistant", "Assistent"),
    ("Show the Nora assistant pane (experimental)", "Den Nora-Assistenten-Bereich anzeigen (experimentell)"),
    ("Nora assistant", "Nora-Assistent"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "Der Befehl `{command}` wird abgelehnt: Nur du selbst darfst deine Unterschrift und deine Initialen hinzufügen.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "Der Befehl `{command}` wird abgelehnt: Die Sicherheitseinstellungen dieses Dokuments verbieten das Kopieren seines Textes.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Noras Änderung ist zu alt, um sie als Ganzes rückgängig zu machen: Verwende stattdessen Rückgängig im Editor.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Stoppen"),
    ("Stopped.", "Gestoppt."),
    ("Nora did not answer in time.", "Nora hat nicht rechtzeitig geantwortet."),
    (
        "Nora could not be reached. Check the connection and try again.",
        "Nora war nicht erreichbar. Prüfen Sie die Verbindung und versuchen Sie es erneut.",
    ),
    ("Your session has expired. Sign in again.", "Ihre Sitzung ist abgelaufen. Bitte melden Sie sich erneut an."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Der Server konnte nicht antworten (Fehler {status}). Versuchen Sie es in einem Moment erneut.",
    ),
    ("Nora's answer could not be read.", "Die Antwort von Nora konnte nicht gelesen werden."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Dieser Tab war im Hintergrund: Nora konnte im Dokument nicht handeln."),
    ("Resume", "Fortsetzen"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Im Dokument wurde nichts ausgeführt: Nora konnte rechtzeitig nicht darin handeln. Frag noch einmal, falls du es noch brauchst.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Nicht alles wurde ausgeführt: ein Teil davon konnte im Dokument rechtzeitig nicht laufen.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Nora fragen"),
    ("Nora, the Neoffice assistant", "Nora, die Neoffice-Assistentin"),
];
