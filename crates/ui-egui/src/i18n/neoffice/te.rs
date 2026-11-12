// //// Neoffice — added file (no upstream equivalent)
//! Telugu words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "ఈ వెర్షన్లో ఇంకా Nora అందుబాటులో లేదు"),
    ("Nora is waiting for you to finish what you are doing in the document.", "పత్రంలో మీరు చేస్తున్న పని పూర్తయ్యే వరకు Nora వేచి ఉంది."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "ఈ కార్డ్‌కు Neoffice లోని Nora చాట్‌లో సమాధానం ఇవ్వండి: NeoPDF ఇంకా దీన్ని స్వీకరించలేదు.",
    ),
    ("There is no document open for Nora to work in.", "Nora పని చేయడానికి తెరిచి ఉన్న పత్రం ఏదీ లేదు."),
    ("`{command}` is refused: Nora may not run it.", "`{command}` తిరస్కరించబడింది: దీన్ని Nora అమలు చేయడానికి అనుమతి లేదు."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "NeoPDF లో `{command}` ను Nora అమలు చేయలేరు: ఇది ఇంటర్ఫేస్ ద్వారా మాత్రమే పనిచేస్తుంది.",
    ),
    ("`{command}` is played by several tools ({tools}): name the one to run.", "`{command}` అనేక సాధనాల్లో ఉంది ({tools}): అమలు చేయాల్సిన సాధనాన్ని పేర్కొనండి."),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` NeoPDF ఆదేశం కాదు: Nora అమలు చేయగల ఆదేశాలను command_list జాబితా చేస్తుంది.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "`{command}` తిరస్కరించబడింది: దాని పారామీటర్లు చాలా లోతుగా నెస్ట్ చేయబడ్డాయి."),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "`{command}` తిరస్కరించబడింది: `{key}` కంప్యూటర్లోని ఒక ఫైల్‌ను యాక్సెస్ చేస్తుంది."),
    ("`{command}` is refused: `{key}` is a password, which only you enter.", "`{command}` తిరస్కరించబడింది: `{key}` ఒక పాస్‌వర్డ్; దాన్ని మీరు మాత్రమే నమోదు చేయగలరు."),
    ("The parameters of `{command}` must be an object.", "`{command}` యొక్క పారామీటర్లు తప్పనిసరిగా ఒక ఆబ్జెక్ట్ అయి ఉండాలి."),
    ("Nora works in the document open in this window: `doc` names another one.", "Nora ఈ విండోలో తెరిచి ఉన్న పత్రంలో పనిచేస్తుంది: `doc` వేరే పత్రాన్ని పేర్కొంటుంది."),
    ("`{command}` failed unexpectedly ({error}); the document was not changed.", "`{command}` ఊహించని విధంగా విఫలమైంది ({error}); పత్రం మార్చబడలేదు."),
    ("Commands in one batch: {count} at most.", "ఒక బ్యాచ్‌లోని ఆదేశాలు: గరిష్ఠంగా {count}."),
    ("`{command}` failed: {error}", "`{command}` విఫలమైంది: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "పత్రంలో “{text}” కనిపించిన సార్లు: {count} ({occurrence} కాదు)."),
    ("“{text}” is not in the first {pages} pages of the document.", "“{text}” పత్రంలోని మొదటి {pages} పేజీల్లో లేదు."),
    ("“{text}” is not in the document.", "“{text}” పత్రంలో లేదు."),
    ("There is no text to type.", "టైప్ చేయడానికి టెక్స్ట్ ఏదీ లేదు."),
    ("There is no page {page}: the last page is {count}.", "పేజీ {page} లేదు: చివరి పేజీ {count}."),
    ("{name} was not saved: {why}", "{name} సేవ్ కాలేదు: {why}"),
    ("Nora has nothing left to undo.", "రద్దు చేయడానికి Nora వద్ద ఇంకేమీ లేదు."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Nora చేసిన చివరి మార్పు నుండి పత్రం మారింది: బదులుగా ఎడిటర్‌లో రద్దు చేయిని ఉపయోగించండి.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "రద్దు చేయడం విఫలమైంది ({count}లో {step}వ దశ): {error}"),
    ("New conversation", "కొత్త సంభాషణ"),
    ("Undo the last change Nora made to the document", "Nora పత్రంలో చేసిన చివరి మార్పును రద్దు చేయి"),
    ("Ask Nora…", "Nora ను అడగండి…"),
    ("Send", "పంపు"),
    ("Nora is working…", "Nora పని చేస్తోంది…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "మీ పత్రాన్ని చదవమని, సారాంశం ఇవ్వమని, నింపమని లేదా గమనికలు జోడించమని Nora ను అడగండి."),
    ("Nora wants to:", "Nora ఈ పనులు చేయాలనుకుంటోంది:"),
    ("Refuse", "తిరస్కరించు"),
    ("Read the document", "పత్రాన్ని చదవడం"),
    ("Run {command}", "{command} ను అమలు చేయడం"),
    ("Run several commands ({count})", "అనేక ఆదేశాలను అమలు చేయడం ({count})"),
    ("Select “{text}”", "“{text}”ను ఎంచుకోవడం"),
    ("Type “{text}”", "“{text}”ను టైప్ చేయడం"),
    ("Look at page {page}", "పేజీ {page}ను పరిశీలించడం"),
    ("Save the document", "పత్రాన్ని సేవ్ చేయడం"),
    ("Undo the last change", "చివరి మార్పును రద్దు చేయడం"),
    ("Assistant", "సహాయకుడు"),
    ("Show the Nora assistant pane (experimental)", "Nora సహాయకుడు ప్యానెల్‌ను చూపించు (ప్రయోగాత్మకం)"),
    ("Nora assistant", "Nora సహాయకుడు"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "`{command}` తిరస్కరించబడింది: మీ సంతకం మరియు పొడి అక్షరాలను జోడించే హక్కు మీకు మాత్రమే ఉంది.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` తిరస్కరించబడింది: ఈ పత్రం భద్రతా సెట్టింగ్‌లు దాని టెక్స్ట్‌ను కాపీ చేయడాన్ని నిషేధిస్తాయి.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Nora చేసిన మార్పు మొత్తంగా రద్దు చేయడానికి చాలా పాతది: బదులుగా ఎడిటర్‌లో రద్దు చేయిని ఉపయోగించండి.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "ఆపు"),
    ("Stopped.", "ఆపివేయబడింది."),
    ("Nora did not answer in time.", "Nora సమయానికి స్పందించలేదు."),
    ("Nora could not be reached. Check the connection and try again.", "Nora కు కనెక్ట్ కాలేకపోయింది. కనెక్షన్‌ను తనిఖీ చేసి, మళ్లీ ప్రయత్నించండి."),
    ("Your session has expired. Sign in again.", "మీ సెషన్ గడువు ముగిసింది. దయచేసి మళ్లీ సైన్ ఇన్ చేయండి."),
    ("The server could not answer (error {status}). Try again in a moment.", "సర్వర్ స్పందించలేకపోయింది (లోపం {status}). కొద్దిసేపటి తర్వాత మళ్లీ ప్రయత్నించండి."),
    ("Nora's answer could not be read.", "Nora సమాధానాన్ని చదవడం సాధ్యం కాలేదు."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "ఈ ట్యాబ్ నేపథ్యంలో ఉంది: Nora పత్రంలో చర్య తీసుకోలేకపోయారు."),
    ("Resume", "మళ్లీ ప్రారంభించు"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "పత్రంలో ఏమీ జరగలేదు: Nora సమయానికి అందులో చర్య తీసుకోలేకపోయారు. ఇంకా అవసరమైతే మళ్లీ అడగండి.",
    ),
    ("Not all of this was done: part of it could not run in the document in time.", "ఇందులో కొంత భాగమే జరిగింది: అందులోని కొంత భాగం సమయానికి పత్రంలో నడవలేకపోయింది."),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Nora ని అడగండి"),
    ("Nora, the Neoffice assistant", "Nora, Neoffice సహాయకురాలు"),
];
