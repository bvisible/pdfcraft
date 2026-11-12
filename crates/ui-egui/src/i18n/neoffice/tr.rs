// //// Neoffice — added file (no upstream equivalent)
//! Turkish words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora bu sürümde henüz kullanılamıyor"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora, belgede yaptığınız işi bitirmenizi bekliyor."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Bu kartı Neoffice içindeki Nora sohbetinde yanıtlayın: NeoPDF bunu henüz destekleyemiyor.",
    ),
    ("There is no document open for Nora to work in.", "Nora'nın üzerinde çalışabileceği açık bir belge yok."),
    ("`{command}` is refused: Nora may not run it.", "`{command}` komutu reddedildi: Nora bu komutu çalıştırmaya yetkili değil."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "`{command}` komutu NeoPDF'de Nora tarafından çalıştırılamaz: yalnızca arayüz üzerinden çalışır.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "`{command}` komutu birden çok araçta bulunuyor ({tools}): çalıştırılacak aracı belirtin.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}`, NeoPDF'in bir komutu değil: command_list, Nora'nın çalıştırabileceği komutları listeler.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "`{command}` komutu reddedildi: parametreleri çok fazla iç içe geçmiş."),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "`{command}` komutu reddedildi: `{key}` bilgisayardaki bir dosyaya erişirdi.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "`{command}` komutu reddedildi: `{key}` bir parola; onu yalnızca siz girersiniz.",
    ),
    ("The parameters of `{command}` must be an object.", "`{command}` komutunun parametreleri bir nesne olmalıdır."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora, bu pencerede açık olan belgede çalışır: `doc` başka bir belgeyi belirtir.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "`{command}` komutu beklenmedik şekilde başarısız oldu ({error}); belge değiştirilmedi.",
    ),
    ("Commands in one batch: {count} at most.", "Tek bir toplu işlemdeki komutlar: en fazla {count}."),
    ("`{command}` failed: {error}", "`{command}` komutu başarısız oldu: {error}"),
    (
        "Occurrences of “{text}” in the document: {count} (not {occurrence}).",
        "Belgede “{text}” ifadesinin geçme sayısı: {count} (değil {occurrence}).",
    ),
    ("“{text}” is not in the first {pages} pages of the document.", "“{text}” belgenin ilk {pages} sayfasında yok."),
    ("“{text}” is not in the document.", "“{text}” belgede yok."),
    ("There is no text to type.", "Yazılacak metin yok."),
    ("There is no page {page}: the last page is {count}.", "Sayfa {page} yok: son sayfa {count}."),
    ("{name} was not saved: {why}", "{name} dosyası kaydedilmedi: {why}"),
    ("Nora has nothing left to undo.", "Nora'nın geri alınacak bir değişikliği kalmadı."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Belge, Nora'nın son değişikliğinden bu yana değişti: bunun yerine düzenleyicideki Geri al komutunu kullanın.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Geri alma, {step}/{count} adımında başarısız oldu: {error}"),
    ("New conversation", "Yeni sohbet"),
    ("Undo the last change Nora made to the document", "Nora'nın belgede yaptığı son değişikliği geri al"),
    ("Ask Nora…", "Nora'ya sorun…"),
    ("Send", "Gönder"),
    ("Nora is working…", "Nora çalışıyor…"),
    (
        "Ask Nora to read, summarize, fill in or annotate your document.",
        "Nora'dan belgenizi okumasını, özetlemesini, doldurmasını veya not eklemesini isteyin.",
    ),
    ("Nora wants to:", "Nora şunu yapmak istiyor:"),
    ("Refuse", "Reddet"),
    ("Read the document", "Belgeyi okumak"),
    ("Run {command}", "{command} komutunu çalıştırmak"),
    ("Run several commands ({count})", "Birden çok komutu çalıştırmak ({count})"),
    ("Select “{text}”", "“{text}” seçmek"),
    ("Type “{text}”", "“{text}” yazmak"),
    ("Look at page {page}", "Sayfa {page} görüntülemek"),
    ("Save the document", "Belgeyi kaydetmek"),
    ("Undo the last change", "Son değişikliği geri almak"),
    ("Assistant", "Asistan"),
    ("Show the Nora assistant pane (experimental)", "Nora asistan panelini göster (deneysel)"),
    ("Nora assistant", "Nora asistanı"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "`{command}` komutu reddedildi: imzanızı ve parafınızı yalnızca siz ekleyebilirsiniz.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` komutu reddedildi: bu belgenin güvenlik ayarları metninin kopyalanmasını yasaklıyor.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Nora'nın değişikliği bütünüyle geri alınamayacak kadar eski: bunun yerine düzenleyicideki Geri al komutunu kullanın.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Durdur"),
    ("Stopped.", "Durduruldu."),
    ("Nora did not answer in time.", "Nora zamanında yanıt vermedi."),
    ("Nora could not be reached. Check the connection and try again.", "Nora'ya bağlanılamadı. Bağlantınızı kontrol edip yeniden deneyin."),
    ("Your session has expired. Sign in again.", "Oturumunuzun süresi doldu. Lütfen yeniden oturum açın."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Sunucu yanıt veremedi (hata {status}). Lütfen birazdan yeniden deneyin.",
    ),
    ("Nora's answer could not be read.", "Nora'nın yanıtı okunamadı."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Bu sekme arka plandaydı: Nora belgede işlem yapamadı."),
    ("Resume", "Devam et"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Belgede hiçbir şey yapılmadı: Nora zamanında işlem yapamadı. Hâlâ gerekiyorsa yeniden isteyin.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Bunların tümü yapılmadı: bir kısmı zamanında belgede çalıştırılamadı.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Nora'ya sorun"),
    ("Nora, the Neoffice assistant", "Nora, Neoffice asistanı"),
];
