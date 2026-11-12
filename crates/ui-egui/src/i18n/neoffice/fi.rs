// //// Neoffice — added file (no upstream equivalent)
//! Finnish words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora ei ole vielä saatavilla tässä versiossa"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora odottaa, että lopetat dokumentissa tekemäsi työn."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Vastaa tähän korttiin Noran keskustelussa Neoffice-sovelluksessa: NeoPDF ei vielä osaa käsitellä vastausta.",
    ),
    ("There is no document open for Nora to work in.", "Dokumenttia, jossa Nora voisi työskennellä, ei ole avoinna."),
    ("`{command}` is refused: Nora may not run it.", "`{command}` on estetty: Nora ei saa suorittaa sitä."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "Nora ei voi suorittaa komentoa `{command}` NeoPDF-ohjelmassa: se toimii vain käyttöliittymän kautta.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "`{command}` löytyy useista työkaluista ({tools}): nimeä se, joka suoritetaan.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` ei ole NeoPDF-sovelluksen komento: command_list luettelee komennot, joita Nora voi suorittaa.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "`{command}` on estetty: sen parametrit ovat liian syvälle sisäkkäin."),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "`{command}` on estetty: `{key}` ulottuisi tietokoneen tiedostoon."),
    ("`{command}` is refused: `{key}` is a password, which only you enter.", "`{command}` on estetty: `{key}` on salasana, jonka vain sinä syötät."),
    ("The parameters of `{command}` must be an object.", "Komennon `{command}` parametrien on oltava olio."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora työskentelee tässä ikkunassa avoinna olevassa asiakirjassa: `doc` nimeää toisen asiakirjan.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "`{command}` epäonnistui odottamatta ({error}): asiakirjaa ei muutettu.",
    ),
    ("Commands in one batch: {count} at most.", "Komentoja yhdessä erässä: enintään {count}."),
    ("`{command}` failed: {error}", "`{command}` epäonnistui: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Tekstin ”{text}” esiintymät asiakirjassa: {count} (ei {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "Teksti ”{text}” ei ole asiakirjan {pages} ensimmäisellä sivulla."),
    ("“{text}” is not in the document.", "Teksti ”{text}” ei ole asiakirjassa."),
    ("There is no text to type.", "Kirjoitettavaa tekstiä ei ole."),
    ("There is no page {page}: the last page is {count}.", "Sivua {page} ei ole: viimeinen sivu on {count}."),
    ("{name} was not saved: {why}", "Tiedostoa {name} ei tallennettu: {why}"),
    ("Nora has nothing left to undo.", "Nora ei ole enää mitään kumottavaa."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Asiakirjaa on muutettu sen jälkeen, kun Nora teki viimeisen muutoksensa: käytä sen sijaan editorin Kumoa-toimintoa.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Kumoaminen epäonnistui vaiheessa {step}/{count}: {error}"),
    ("New conversation", "Uusi keskustelu"),
    ("Undo the last change Nora made to the document", "Kumoa viimeisin muutos, jonka Nora teki asiakirjaan"),
    ("Ask Nora…", "Kysy Nora-avustajalta…"),
    ("Send", "Lähetä"),
    ("Nora is working…", "Nora työskentelee…"),
    (
        "Ask Nora to read, summarize, fill in or annotate your document.",
        "Pyydä Nora-avustajaa lukemaan, tiivistämään, täyttämään tai merkitsemään asiakirjasi.",
    ),
    ("Nora wants to:", "Nora haluaa:"),
    ("Refuse", "Kiellä"),
    ("Read the document", "Lue asiakirja"),
    ("Run {command}", "Suorita {command}"),
    ("Run several commands ({count})", "Suorita useita komentoja ({count})"),
    ("Select “{text}”", "Valitse ”{text}”"),
    ("Type “{text}”", "Kirjoita ”{text}”"),
    ("Look at page {page}", "Katso sivua {page}"),
    ("Save the document", "Tallenna asiakirja"),
    ("Undo the last change", "Kumoa viimeisin muutos"),
    ("Assistant", "Avustaja"),
    ("Show the Nora assistant pane (experimental)", "Näytä Nora-avustajan paneeli (kokeellinen)"),
    ("Nora assistant", "Nora-avustaja"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "`{command}` on estetty: allekirjoituksesi ja nimikirjaimesi lisäät itse.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` on estetty: asiakirjan suojaus kieltää tekstin kopioimisen.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Nora-muutos on liian vanha kumottavaksi kokonaan: käytä sen sijaan editorin Kumoa-toimintoa.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Pysäytä"),
    ("Stopped.", "Pysäytetty."),
    ("Nora did not answer in time.", "Nora ei vastannut ajoissa."),
    ("Nora could not be reached. Check the connection and try again.", "Nora ei ollut tavoitettavissa. Tarkista yhteys ja yritä uudelleen."),
    ("Your session has expired. Sign in again.", "Istuntosi on vanhentunut. Kirjaudu uudelleen sisään."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Palvelin ei voinut vastata (virhe {status}). Yritä hetken kuluttua uudelleen.",
    ),
    ("Nora's answer could not be read.", "Nora-avustajan vastausta ei voitu lukea."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Tämä välilehti oli taustalla: Nora ei voinut toimia asiakirjassa."),
    ("Resume", "Jatka"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Asiakirjassa ei tehty mitään: Nora ei ehtinyt toimia siinä ajoissa. Pyydä uudelleen, jos tarvitset sitä vielä.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Kaikkea ei tehty: osa siitä ei ehtinyt toteutua asiakirjassa ajoissa.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Kysy Noralta"),
    ("Nora, the Neoffice assistant", "Nora, Neoffice-avustaja"),
];
