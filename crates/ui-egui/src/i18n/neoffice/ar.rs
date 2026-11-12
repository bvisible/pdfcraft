// //// Neoffice — added file (no upstream equivalent)
//! Arabic words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora غير متاح في هذا الإصدار بعد"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora في انتظار انتهائك مما تقوم به في المستند."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "أجب عن هذه البطاقة في دردشة Nora داخل Neoffice: NeoPDF لا يستطيع معالجتها بعد.",
    ),
    ("There is no document open for Nora to work in.", "لا يوجد مستند مفتوح ليعمل فيه Nora."),
    ("`{command}` is refused: Nora may not run it.", "تم رفض `{command}`: لا يُسمح لـ Nora بتشغيله."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "لا يمكن لـ Nora تشغيل `{command}` في NeoPDF: لا يعمل هذا الأمر إلا عبر الواجهة.",
    ),
    ("`{command}` is played by several tools ({tools}): name the one to run.", "`{command}` متوفر في عدة أدوات ({tools}): حدِّد الأداة المراد تشغيلها."),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` ليس أمرًا من أوامر NeoPDF: تعرض command_list الأوامر التي يمكن لـ Nora تشغيلها.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "تم رفض `{command}`: المعاملات متداخلة بعمق زائد."),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "تم رفض `{command}`: سيؤدي `{key}` إلى الوصول إلى ملف على الحاسوب."),
    ("`{command}` is refused: `{key}` is a password, which only you enter.", "تم رفض `{command}`: `{key}` كلمة مرور، ولا يُدخلها إلا أنت."),
    ("The parameters of `{command}` must be an object.", "يجب أن تكون معاملات `{command}` كائنًا."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "يعمل Nora في المستند المفتوح في هذه النافذة: يحدِّد `doc` مستندًا آخر.",
    ),
    ("`{command}` failed unexpectedly ({error}); the document was not changed.", "فشل `{command}` بشكل غير متوقع ({error})؛ لم يتغير المستند."),
    ("Commands in one batch: {count} at most.", "الحد الأقصى للأوامر في الدفعة الواحدة: {count}."),
    ("`{command}` failed: {error}", "فشل `{command}`: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "عدد مرات ورود «{text}» في المستند: {count} (وليس {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "«{text}» غير موجود في الصفحات الأولى من المستند (حتى الصفحة {pages})."),
    ("“{text}” is not in the document.", "«{text}» غير موجود في المستند."),
    ("There is no text to type.", "لا يوجد نص للكتابة."),
    ("There is no page {page}: the last page is {count}.", "لا توجد صفحة {page}: آخر صفحة هي الصفحة {count}."),
    ("{name} was not saved: {why}", "لم يُحفظ {name}: {why}"),
    ("Nora has nothing left to undo.", "لم يبقَ لدى Nora ما يمكن التراجع عنه."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "تغيّر المستند منذ آخر تغيير أجراه Nora: استخدم «تراجع» في المحرر بدلًا من ذلك.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "فشل التراجع في الخطوة {step} من {count}: {error}"),
    ("New conversation", "محادثة جديدة"),
    ("Undo the last change Nora made to the document", "التراجع عن آخر تغيير أجراه Nora في المستند"),
    ("Ask Nora…", "اسأل Nora…"),
    ("Send", "إرسال"),
    ("Nora is working…", "Nora يعمل…"),
    (
        "Ask Nora to read, summarize, fill in or annotate your document.",
        "اطلب من Nora قراءة مستندك أو تلخيصه أو تعبئة حقوله أو إضافة تعليقات توضيحية إليه.",
    ),
    ("Nora wants to:", "يطلب Nora الإذن بما يلي:"),
    ("Refuse", "رفض"),
    ("Read the document", "قراءة المستند"),
    ("Run {command}", "تشغيل {command}"),
    ("Run several commands ({count})", "تشغيل عدة أوامر ({count})"),
    ("Select “{text}”", "تحديد «{text}»"),
    ("Type “{text}”", "كتابة «{text}»"),
    ("Look at page {page}", "الاطلاع على الصفحة {page}"),
    ("Save the document", "حفظ المستند"),
    ("Undo the last change", "التراجع عن آخر تغيير"),
    ("Assistant", "المساعد"),
    ("Show the Nora assistant pane (experimental)", "عرض لوحة مساعد Nora (تجريبي)"),
    ("Nora assistant", "مساعد Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    ("`{command}` is refused: your signature and initials are yours to add.", "تم رفض `{command}`: إضافة توقيعك وأحرفك الأولى تعود إليك وحدك."),
    ("`{command}` is refused: this document's security forbids copying its text.", "تم رفض `{command}`: إعدادات أمان هذا المستند تمنع نسخ نصه."),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "تغيير Nora قديم جدًا فلا يمكن التراجع عنه كاملًا: استخدم «تراجع» في المحرر بدلًا من ذلك.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "إيقاف"),
    ("Stopped.", "تم الإيقاف."),
    ("Nora did not answer in time.", "انتهت المهلة الزمنية دون رد من Nora."),
    ("Nora could not be reached. Check the connection and try again.", "تعذّر الاتصال بـ Nora. تحقق من الاتصال وحاول مرة أخرى."),
    ("Your session has expired. Sign in again.", "انتهت صلاحية جلستك. يرجى تسجيل الدخول مرة أخرى."),
    ("The server could not answer (error {status}). Try again in a moment.", "تعذّر على الخادم الرد (خطأ {status}). يرجى المحاولة مرة أخرى بعد قليل."),
    ("Nora's answer could not be read.", "تعذّرت قراءة رد Nora."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "هذه علامة التبويب كانت في الخلفية: لم تستطع Nora التصرف في المستند."),
    ("Resume", "استئناف"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "لم يُنجَز شيء في المستند: لم تستطع Nora التصرف فيه في الوقت المناسب. اطلب مجددًا إن كنت ما زلت تحتاجه.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "لم يُنجَز كل هذا: جزء منه لم يتمكن من التنفيذ في المستند في الوقت المناسب.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "اسأل Nora"),
    ("Nora, the Neoffice assistant", "Nora، مساعدة Neoffice"),
];
