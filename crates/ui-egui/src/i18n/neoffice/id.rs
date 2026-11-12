// //// Neoffice — added file (no upstream equivalent)
//! Indonesian words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora belum tersedia di versi ini"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora menunggu Anda menyelesaikan pekerjaan di dokumen."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Jawab kartu ini di obrolan Nora di Neoffice: NeoPDF belum dapat memprosesnya.",
    ),
    ("There is no document open for Nora to work in.", "Tidak ada dokumen terbuka untuk tempat Nora bekerja."),
    ("`{command}` is refused: Nora may not run it.", "`{command}` ditolak: Nora tidak boleh menjalankannya."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "`{command}` tidak dapat dijalankan oleh Nora di NeoPDF: perintah ini hanya berfungsi lewat antarmuka.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "`{command}` tersedia di beberapa alat ({tools}): sebutkan alat yang akan dijalankan.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` bukan perintah NeoPDF: command_list mencantumkan perintah yang dapat dijalankan Nora.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "`{command}` ditolak: parameternya bersarang terlalu dalam."),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "`{command}` ditolak: `{key}` akan mengarah ke berkas di komputer."),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "`{command}` ditolak: `{key}` adalah kata sandi, yang hanya Anda masukkan.",
    ),
    ("The parameters of `{command}` must be an object.", "Parameter `{command}` harus berupa objek."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora bekerja di dokumen yang terbuka di jendela ini: `doc` menunjuk dokumen lain.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "`{command}` gagal secara tak terduga ({error}); dokumen tidak diubah.",
    ),
    ("Commands in one batch: {count} at most.", "Perintah dalam satu kelompok: maksimal {count}."),
    ("`{command}` failed: {error}", "`{command}` gagal: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Kemunculan “{text}” dalam dokumen: {count} (bukan {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "“{text}” tidak ada di {pages} halaman pertama dokumen."),
    ("“{text}” is not in the document.", "“{text}” tidak ada di dokumen."),
    ("There is no text to type.", "Tidak ada teks untuk diketik."),
    ("There is no page {page}: the last page is {count}.", "Tidak ada halaman {page}: halaman terakhir adalah {count}."),
    ("{name} was not saved: {why}", "{name} tidak disimpan: {why}"),
    ("Nora has nothing left to undo.", "Nora tidak punya apa pun lagi untuk diurungkan."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Dokumen telah berubah sejak perubahan terakhir Nora: gunakan Urungkan di editor sebagai gantinya.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Urungkan gagal pada langkah {step} dari {count}: {error}"),
    ("New conversation", "Percakapan baru"),
    ("Undo the last change Nora made to the document", "Urungkan perubahan terakhir yang dibuat Nora pada dokumen"),
    ("Ask Nora…", "Tanyakan kepada Nora…"),
    ("Send", "Kirim"),
    ("Nora is working…", "Nora sedang bekerja…"),
    (
        "Ask Nora to read, summarize, fill in or annotate your document.",
        "Minta Nora untuk membaca, merangkum, mengisi, atau memberi anotasi pada dokumen Anda.",
    ),
    ("Nora wants to:", "Nora ingin:"),
    ("Refuse", "Tolak"),
    ("Read the document", "Baca dokumen"),
    ("Run {command}", "Jalankan {command}"),
    ("Run several commands ({count})", "Jalankan beberapa perintah ({count})"),
    ("Select “{text}”", "Pilih “{text}”"),
    ("Type “{text}”", "Ketik “{text}”"),
    ("Look at page {page}", "Lihat halaman {page}"),
    ("Save the document", "Simpan dokumen"),
    ("Undo the last change", "Urungkan perubahan terakhir"),
    ("Assistant", "Asisten"),
    ("Show the Nora assistant pane (experimental)", "Tampilkan panel asisten Nora (eksperimental)"),
    ("Nora assistant", "Asisten Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "`{command}` ditolak: hanya Anda yang boleh menambahkan tanda tangan dan inisial Anda.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` ditolak: keamanan dokumen ini melarang penyalinan teksnya.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Perubahan Nora terlalu lama untuk diurungkan seluruhnya: gunakan Urungkan di editor sebagai gantinya.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Hentikan"),
    ("Stopped.", "Dihentikan."),
    ("Nora did not answer in time.", "Nora tidak menjawab tepat waktu."),
    ("Nora could not be reached. Check the connection and try again.", "Nora tidak dapat dihubungi. Periksa koneksi lalu coba lagi."),
    ("Your session has expired. Sign in again.", "Sesi Anda telah berakhir. Masuk lagi."),
    ("The server could not answer (error {status}). Try again in a moment.", "Server tidak dapat merespons (galat {status}). Coba lagi sebentar."),
    ("Nora's answer could not be read.", "Jawaban Nora tidak dapat dibaca."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Tab ini berada di latar belakang: Nora tidak dapat beraksi di dokumen."),
    ("Resume", "Lanjutkan"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Tidak ada yang dilakukan di dokumen: Nora tidak dapat bertindak tepat waktu. Tanyakan lagi jika Anda masih membutuhkannya.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Tidak semuanya dilakukan: sebagian tidak dapat dijalankan di dokumen tepat waktu.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Tanya Nora"),
    ("Nora, the Neoffice assistant", "Nora, asisten Neoffice"),
];
