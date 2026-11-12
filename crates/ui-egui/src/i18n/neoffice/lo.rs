// //// Neoffice — added file (no upstream equivalent)
//! Lao words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora ຍັງບໍ່ມີໃຫ້ໃຊ້ໃນເວີຊັນນີ້"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora ກຳລັງລໍຖ້າໃຫ້ທ່ານເຮັດສິ່ງທີ່ກຳລັງເຮັດຢູ່ໃນເອກະສານໃຫ້ສຳເລັດກ່ອນ."),
    ("Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.", "ກະລຸນາຕອບການ໌ດນີ້ໃນແຊັດ Nora ໃນ Neoffice: NeoPDF ຍັງບໍ່ສາມາດຮັບການຕອບນີ້ໄດ້."),
    ("There is no document open for Nora to work in.", "ບໍ່ມີເອກະສານໃດເປີດຢູ່ໃຫ້ Nora ເຮັດວຽກໄດ້."),
    ("`{command}` is refused: Nora may not run it.", "ຄຳສັ່ງ `{command}` ຖືກປະຕິເສດ: Nora ບໍ່ມີສິດເອີ້ນໃຊ້ຄຳສັ່ງນີ້."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "ຄຳສັ່ງ `{command}` ເອີ້ນໃຊ້ໂດຍ Nora ໃນ NeoPDF ບໍ່ໄດ້: ມັນໃຊ້ໄດ້ພຽງແຕ່ຜ່ານສ່ວນຕິດຕໍ່ຜູ້ໃຊ້ເທົ່ານັ້ນ.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "ຄຳສັ່ງ `{command}` ຖືກສະໜອງໃຫ້ໂດຍຫຼາຍເຄື່ອງມື ({tools}): ກະລຸນາລະບຸເຄື່ອງມືທີ່ຈະເອີ້ນໃຊ້.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` ບໍ່ແມ່ນຄຳສັ່ງຂອງ NeoPDF: command_list ລາຍການຄຳສັ່ງທີ່ Nora ສາມາດເອີ້ນໃຊ້ໄດ້.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "ຄຳສັ່ງ `{command}` ຖືກປະຕິເສດ: ພາລາມິເຕີຂອງມັນຊ້ອນກັນເລິກເກີນໄປ."),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "ຄຳສັ່ງ `{command}` ຖືກປະຕິເສດ: `{key}` ຈະເຂົ້າເຖິງໄຟລ໌ໃນຄອມພິວເຕີ."),
    ("`{command}` is refused: `{key}` is a password, which only you enter.", "ຄຳສັ່ງ `{command}` ຖືກປະຕິເສດ: `{key}` ເປັນລະຫັດຜ່ານ ທີ່ມີແຕ່ທ່ານເທົ່ານັ້ນທີ່ປ້ອນໄດ້."),
    ("The parameters of `{command}` must be an object.", "ພາລາມິເຕີຂອງ `{command}` ຕ້ອງເປັນອອບເຈັກ."),
    ("Nora works in the document open in this window: `doc` names another one.", "Nora ເຮັດວຽກກັບເອກະສານທີ່ເປີດຢູ່ໃນໜ້າຕ່າງນີ້: `doc` ລະບຸເອກະສານອື່ນ."),
    ("`{command}` failed unexpectedly ({error}); the document was not changed.", "ຄຳສັ່ງ `{command}` ລົ້ມເຫຼວຢ່າງບໍ່ຄາດຄິດ ({error}); ເອກະສານບໍ່ໄດ້ຖືກປ່ຽນແປງ."),
    ("Commands in one batch: {count} at most.", "ຄຳສັ່ງໃນຊຸດດຽວກັນ: ສູງສຸດ {count} ຄຳສັ່ງ."),
    ("`{command}` failed: {error}", "ຄຳສັ່ງ `{command}` ລົ້ມເຫຼວ: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "ພົບ “{text}” ໃນເອກະສານ {count} ຄັ້ງ (ບໍ່ແມ່ນ {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "ບໍ່ພົບ “{text}” ໃນ {pages} ໜ້າທຳອິດຂອງເອກະສານ."),
    ("“{text}” is not in the document.", "ບໍ່ພົບ “{text}” ໃນເອກະສານ."),
    ("There is no text to type.", "ບໍ່ມີຂໍ້ຄວາມໃຫ້ປ້ອນ."),
    ("There is no page {page}: the last page is {count}.", "ບໍ່ມີໜ້າ {page}: ໜ້າສຸດທ້າຍແມ່ນໜ້າ {count}."),
    ("{name} was not saved: {why}", "ຍັງບໍ່ໄດ້ບັນທຶກໄຟລ໌ {name}: {why}"),
    ("Nora has nothing left to undo.", "Nora ບໍ່ມີການປ່ຽນແປງເຫຼືອໃຫ້ຍົກເລີກແລ້ວ."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "ເອກະສານໄດ້ປ່ຽນແປງນັບຕັ້ງແຕ່ການປ່ຽນແປງຄັ້ງລ່າສຸດຂອງ Nora: ກະລຸນາໃຊ້ຍົກເລີກການກະທຳໃນຕົວແກ້ໄຂແທນ.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "ການຍົກເລີກລົ້ມເຫຼວຢູ່ຂັ້ນຕອນ {step} ຈາກ {count}: {error}"),
    ("New conversation", "ການສົນທະນາໃໝ່"),
    ("Undo the last change Nora made to the document", "ຍົກເລີກການປ່ຽນແປງຄັ້ງລ່າສຸດທີ່ Nora ເຮັດໃນເອກະສານ"),
    ("Ask Nora…", "ຖາມ Nora…"),
    ("Send", "ສົ່ງ"),
    ("Nora is working…", "Nora ກຳລັງເຮັດວຽກ…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "ຂໍໃຫ້ Nora ອ່ານ ສະຫຼຸບ ຕື່ມຂໍ້ມູນ ຫຼື ໃສ່ໝາຍເຫດໃນເອກະສານຂອງທ່ານ."),
    ("Nora wants to:", "Nora ຕ້ອງການ:"),
    ("Refuse", "ປະຕິເສດ"),
    ("Read the document", "ອ່ານເອກະສານ"),
    ("Run {command}", "ເອີ້ນໃຊ້ {command}"),
    ("Run several commands ({count})", "ເອີ້ນໃຊ້ຫຼາຍຄຳສັ່ງ ({count})"),
    ("Select “{text}”", "ເລືອກ “{text}”"),
    ("Type “{text}”", "ປ້ອນ “{text}”"),
    ("Look at page {page}", "ເບິ່ງໜ້າ {page}"),
    ("Save the document", "ບັນທຶກເອກະສານ"),
    ("Undo the last change", "ຍົກເລີກການປ່ຽນແປງຄັ້ງລ່າສຸດ"),
    ("Assistant", "ຜູ້ຊ່ວຍ"),
    ("Show the Nora assistant pane (experimental)", "ສະແດງແຜງຜູ້ຊ່ວຍ Nora (ທົດລອງ)"),
    ("Nora assistant", "ຜູ້ຊ່ວຍ Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "ຄຳສັ່ງ `{command}` ຖືກປະຕິເສດ: ລາຍເຊັນ ແລະ ອັກສອນຫຍໍ້ຂອງທ່ານ ມີແຕ່ທ່ານເທົ່ານັ້ນທີ່ເພີ່ມໄດ້.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "ຄຳສັ່ງ `{command}` ຖືກປະຕິເສດ: ການປົກປ້ອງຄວາມປອດໄພຂອງເອກະສານນີ້ຫ້າມຄັດລອກຂໍ້ຄວາມ.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "ການປ່ຽນແປງຂອງ Nora ເກົ່າເກີນໄປທີ່ຈະຍົກເລີກທັງໝົດ: ກະລຸນາໃຊ້ຍົກເລີກການກະທຳໃນຕົວແກ້ໄຂແທນ.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "ຢຸດ"),
    ("Stopped.", "ຢຸດແລ້ວ."),
    ("Nora did not answer in time.", "Nora ບໍ່ໄດ້ຕອບທັນເວລາ."),
    ("Nora could not be reached. Check the connection and try again.", "ບໍ່ສາມາດຕິດຕໍ່ Nora ໄດ້. ກະລຸນາກວດສອບການເຊື່ອມຕໍ່ ແລ້ວລອງໃໝ່."),
    ("Your session has expired. Sign in again.", "ເຊດຊັນຂອງທ່ານໝົດອາຍຸແລ້ວ. ກະລຸນາເຂົ້າສູ່ລະບົບອີກຄັ້ງ."),
    ("The server could not answer (error {status}). Try again in a moment.", "ເຊີຟເວີບໍ່ສາມາດຕອບໄດ້ (ຂໍ້ຜິດພາດ {status}). ລອງໃໝ່ອີກຄັ້ງໃນອີກບໍ່ດົນ."),
    ("Nora's answer could not be read.", "ບໍ່ສາມາດອ່ານຄຳຕອບຂອງ Nora ໄດ້."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "ແທັບນີ້ຢູ່ເບື້ອງຫຼັງ: Nora ບໍ່ສາມາດດຳເນີນການໃນເອກະສານໄດ້."),
    ("Resume", "ສືບຕໍ່"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "ບໍ່ມີສິ່ງໃດຖືກດຳເນີນການໃນເອກະສານ: Nora ບໍ່ສາມາດດຳເນີນການໄດ້ທັນເວລາ. ຖາມອີກຄັ້ງຖ້າທ່ານຍັງຕ້ອງການ.",
    ),
    ("Not all of this was done: part of it could not run in the document in time.", "ບໍ່ໄດ້ດຳເນີນການທັງໝົດ: ບາງສ່ວນບໍ່ສາມາດເຮັດວຽກໃນເອກະສານທັນເວລາ."),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "ຖາມ Nora"),
    ("Nora, the Neoffice assistant", "Nora, ຜູ້ຊ່ວຍ Neoffice"),
];
