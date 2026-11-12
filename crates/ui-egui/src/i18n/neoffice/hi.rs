// //// Neoffice — added file (no upstream equivalent)
//! Hindi words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora अभी इस संस्करण में उपलब्ध नहीं है"),
    ("Nora is waiting for you to finish what you are doing in the document.", "आप दस्तावेज़ में जो कर रहे हैं, Nora उसके पूरा होने की प्रतीक्षा कर रहा है।"),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "इस कार्ड का उत्तर Neoffice में Nora की चैट में दें: NeoPDF अभी इसे स्वीकार नहीं करता।",
    ),
    ("There is no document open for Nora to work in.", "Nora के काम करने के लिए कोई दस्तावेज़ खुला नहीं है।"),
    ("`{command}` is refused: Nora may not run it.", "`{command}` अस्वीकृत है: Nora को इसे चलाने की अनुमति नहीं है।"),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "`{command}` को NeoPDF में Nora नहीं चला सकता: यह केवल इंटरफ़ेस के ज़रिए काम करता है।",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "कई टूल ({tools}) में `{command}` मौजूद है: चलाने के लिए उनमें से एक का नाम बताएँ।",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` NeoPDF का कोई कमांड नहीं है: command_list वे कमांड सूचीबद्ध करती है जिन्हें Nora चला सकता है।",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "`{command}` अस्वीकृत है: इसके पैरामीटर बहुत गहराई तक नेस्टेड हैं।"),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "`{command}` अस्वीकृत है: `{key}` कंप्यूटर की किसी फ़ाइल तक पहुँच जाएगा।"),
    ("`{command}` is refused: `{key}` is a password, which only you enter.", "`{command}` अस्वीकृत है: `{key}` एक पासवर्ड है, जिसे केवल आप दर्ज करते हैं।"),
    ("The parameters of `{command}` must be an object.", "`{command}` के पैरामीटर एक ऑब्जेक्ट होने चाहिए।"),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora इस विंडो में खुले दस्तावेज़ में काम करता है: `doc` किसी और दस्तावेज़ का नाम देता है।",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "`{command}` अप्रत्याशित रूप से विफल रहा ({error}); दस्तावेज़ नहीं बदला गया।",
    ),
    ("Commands in one batch: {count} at most.", "एक बैच में कमांड: अधिकतम {count}।"),
    ("`{command}` failed: {error}", "`{command}` विफल रहा: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "दस्तावेज़ में “{text}” की घटनाएँ: {count} ({occurrence} नहीं)।"),
    ("“{text}” is not in the first {pages} pages of the document.", "“{text}” दस्तावेज़ के पहले {pages} पृष्ठों में नहीं है।"),
    ("“{text}” is not in the document.", "“{text}” दस्तावेज़ में नहीं है।"),
    ("There is no text to type.", "टाइप करने के लिए कोई पाठ नहीं है।"),
    ("There is no page {page}: the last page is {count}.", "पृष्ठ {page} मौजूद नहीं है: आखिरी पृष्ठ {count} है।"),
    ("{name} was not saved: {why}", "{name} सहेजी नहीं गई: {why}"),
    ("Nora has nothing left to undo.", "Nora के पास पूर्ववत करने के लिए कुछ नहीं बचा है।"),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Nora के आखिरी बदलाव के बाद दस्तावेज़ बदल चुका है: इसके बजाय संपादक में पूर्ववत करें का उपयोग करें।",
    ),
    ("Undo failed at step {step} of {count}: {error}", "{count} में से चरण {step} पर पूर्ववत करना विफल रहा: {error}"),
    ("New conversation", "नई बातचीत"),
    ("Undo the last change Nora made to the document", "दस्तावेज़ में Nora के किए गए आखिरी बदलाव को पूर्ववत करें"),
    ("Ask Nora…", "Nora से पूछें…"),
    ("Send", "भेजें"),
    ("Nora is working…", "Nora काम कर रहा है…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Nora से अपने दस्तावेज़ को पढ़ने, सारांश देने, भरने या टिप्पणी जोड़ने के लिए कहें।"),
    ("Nora wants to:", "Nora यह करना चाहता है:"),
    ("Refuse", "अस्वीकार करें"),
    ("Read the document", "दस्तावेज़ पढ़ें"),
    ("Run {command}", "{command} चलाएँ"),
    ("Run several commands ({count})", "कई कमांड चलाएँ ({count})"),
    ("Select “{text}”", "“{text}” चुनें"),
    ("Type “{text}”", "“{text}” टाइप करें"),
    ("Look at page {page}", "पृष्ठ {page} देखें"),
    ("Save the document", "दस्तावेज़ सहेजें"),
    ("Undo the last change", "आखिरी बदलाव पूर्ववत करें"),
    ("Assistant", "सहायक"),
    ("Show the Nora assistant pane (experimental)", "Nora सहायक पैनल दिखाएँ (प्रायोगिक)"),
    ("Nora assistant", "Nora सहायक"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    ("`{command}` is refused: your signature and initials are yours to add.", "`{command}` अस्वीकृत है: आपके हस्ताक्षर और आद्याक्षर केवल आप जोड़ सकते हैं।"),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` अस्वीकृत है: इस दस्तावेज़ की सुरक्षा उसका पाठ कॉपी करने की मनाही करती है।",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Nora का बदलाव पूरा वापस लेने के लिए बहुत पुराना है: इसके बजाय संपादक में पूर्ववत करें का उपयोग करें।",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "रोकें"),
    ("Stopped.", "रोका गया।"),
    ("Nora did not answer in time.", "Nora ने समय पर जवाब नहीं दिया।"),
    ("Nora could not be reached. Check the connection and try again.", "Nora से संपर्क नहीं हो सका। कनेक्शन जाँचें और फिर से प्रयास करें।"),
    ("Your session has expired. Sign in again.", "आपका सत्र समाप्त हो गया है। फिर से साइन इन करें।"),
    ("The server could not answer (error {status}). Try again in a moment.", "सर्वर उत्तर नहीं दे सका (त्रुटि {status})। कुछ देर बाद फिर से प्रयास करें।"),
    ("Nora's answer could not be read.", "Nora का जवाब पढ़ा नहीं जा सका।"),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "यह टैब पृष्ठभूमि में था: Nora दस्तावेज़ में कार्रवाई नहीं कर सकी।"),
    ("Resume", "जारी रखें"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "दस्तावेज़ में कुछ भी नहीं किया गया: Nora समय पर कार्रवाई नहीं कर सकी। अगर अब भी आपको इसकी ज़रूरत है, तो फिर से पूछें।",
    ),
    ("Not all of this was done: part of it could not run in the document in time.", "यह सब पूरा नहीं हुआ: इसका एक हिस्सा समय पर दस्तावेज़ में नहीं चल सका।"),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Nora से पूछें"),
    ("Nora, the Neoffice assistant", "Nora, Neoffice की सहायिका"),
];
