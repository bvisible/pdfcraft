// //// Neoffice — added file (no upstream equivalent)
//! Czech words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora zatím není v této verzi dostupná"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora čeká, až dokončíte, co v dokumentu děláte."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Odpovězte na tuto kartu v chatu Nora v Neoffice: NeoPDF ji zatím nezvládá.",
    ),
    ("There is no document open for Nora to work in.", "Není otevřen žádný dokument, ve kterém by Nora mohla pracovat."),
    ("`{command}` is refused: Nora may not run it.", "Příkaz `{command}` byl zamítnut: Nora jej nesmí spustit."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "Příkaz `{command}` nemůže Nora v NeoPDF spustit: funguje pouze přes rozhraní.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "Příkaz `{command}` je zpracováván několika nástroji ({tools}): určete, který z nich spustit.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` není příkazem NeoPDF: command_list uvádí příkazy, které může Nora spustit.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "Příkaz `{command}` byl zamítnut: jeho parametry jsou příliš hluboko vnořené."),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "Příkaz `{command}` byl zamítnut: přes `{key}` by se dostalo k souboru v počítači.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "Příkaz `{command}` byl zamítnut: `{key}` je heslo, které zadáte pouze vy.",
    ),
    ("The parameters of `{command}` must be an object.", "Parametry příkazu `{command}` musí být objekt."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora pracuje v dokumentu otevřeném v tomto okně: `doc` označuje jiný dokument.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "Příkaz `{command}` neočekávaně selhal ({error}); dokument nebyl změněn.",
    ),
    ("Commands in one batch: {count} at most.", "Příkazy v jedné dávce: nejvýše {count}."),
    ("`{command}` failed: {error}", "Příkaz `{command}` selhal: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Výskyty „{text}“ v dokumentu: {count} (ne {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "V prvních {pages} stranách dokumentu není „{text}“."),
    ("“{text}” is not in the document.", "V dokumentu není „{text}“."),
    ("There is no text to type.", "Není žádný text k zadání."),
    ("There is no page {page}: the last page is {count}.", "Stránka {page} neexistuje. Poslední stránka: {count}."),
    ("{name} was not saved: {why}", "Soubor {name} nebyl uložen: {why}"),
    ("Nora has nothing left to undo.", "Nora už nemá co vrátit."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Dokument se změnil od poslední změny, kterou provedla Nora: místo toho použijte v editoru tlačítko Zpět.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Vrácení selhalo na kroku {step} z {count}: {error}"),
    ("New conversation", "Nová konverzace"),
    ("Undo the last change Nora made to the document", "Vrátit poslední změnu, kterou Nora provedla v dokumentu"),
    ("Ask Nora…", "Zeptejte se asistenta Nora…"),
    ("Send", "Odeslat"),
    ("Nora is working…", "Nora pracuje…"),
    (
        "Ask Nora to read, summarize, fill in or annotate your document.",
        "Požádejte asistenta Nora, aby dokument přečetl, shrnul, vyplnil nebo okomentoval.",
    ),
    ("Nora wants to:", "Nora chce:"),
    ("Refuse", "Odmítnout"),
    ("Read the document", "Přečíst dokument"),
    ("Run {command}", "Spustit {command}"),
    ("Run several commands ({count})", "Spustit několik příkazů ({count})"),
    ("Select “{text}”", "Vybrat „{text}“"),
    ("Type “{text}”", "Napsat „{text}“"),
    ("Look at page {page}", "Podívat se na stránku {page}"),
    ("Save the document", "Uložit dokument"),
    ("Undo the last change", "Vrátit poslední změnu"),
    ("Assistant", "Asistent"),
    ("Show the Nora assistant pane (experimental)", "Zobrazit panel asistenta Nora (experimentální)"),
    ("Nora assistant", "Asistent Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "Příkaz `{command}` byl zamítnut: svůj podpis a své iniciály můžete přidat jen vy sami.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "Příkaz `{command}` byl zamítnut: nastavení zabezpečení tohoto dokumentu zakazuje kopírovat jeho text.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Změna, kterou provedla Nora, je příliš stará na to, aby šla vzít zpět celá: místo toho použijte v editoru tlačítko Zpět.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Zastavit"),
    ("Stopped.", "Zastaveno."),
    ("Nora did not answer in time.", "Nora neodpověděla včas."),
    ("Nora could not be reached. Check the connection and try again.", "Nora není dostupná. Zkontrolujte připojení a zkuste to znovu."),
    ("Your session has expired. Sign in again.", "Vaše relace vypršela. Přihlaste se znovu."),
    ("The server could not answer (error {status}). Try again in a moment.", "Server nemohl odpovědět (chyba {status}). Zkuste to za chvíli znovu."),
    ("Nora's answer could not be read.", "Odpověď Nory nebylo možné přečíst."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Tato karta byla na pozadí: Nora nemohla v dokumentu nic provést."),
    ("Resume", "Obnovit"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "V dokumentu se nic neprovedlo: Nora v něm nemohla včas nic provést. Pokud to ještě potřebujete, zadejte požadavek znovu.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Ne všechno se provedlo: část se v dokumentu včas nepodařilo spustit.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Zeptat se Nory"),
    ("Nora, the Neoffice assistant", "Nora, asistentka Neoffice"),
];
