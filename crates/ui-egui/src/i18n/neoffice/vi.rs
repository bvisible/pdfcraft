// //// Neoffice — added file (no upstream equivalent)
//! Vietnamese words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora chưa có trong phiên bản này."),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora đang chờ bạn hoàn tất thao tác đang làm trong tài liệu."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Hãy trả lời thẻ này trong khung trò chuyện của Nora trong Neoffice: NeoPDF chưa xử lý được nội dung này.",
    ),
    ("There is no document open for Nora to work in.", "Không có tài liệu nào đang mở để Nora làm việc."),
    ("`{command}` is refused: Nora may not run it.", "Lệnh `{command}` bị từ chối: Nora không được phép chạy lệnh này."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "Nora không thể chạy lệnh `{command}` trong NeoPDF: lệnh này chỉ hoạt động qua giao diện.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "Lệnh `{command}` được cung cấp bởi nhiều công cụ ({tools}): hãy chỉ định công cụ cần chạy.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` không phải là lệnh của NeoPDF: command_list liệt kê các lệnh mà Nora có thể chạy.",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "Lệnh `{command}` bị từ chối: các tham số của lệnh lồng nhau quá sâu."),
    ("`{command}` is refused: `{key}` would reach a file of the computer.", "Lệnh `{command}` bị từ chối: `{key}` sẽ trỏ tới một tệp trên máy tính."),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "Lệnh `{command}` bị từ chối: `{key}` là mật khẩu, chỉ bạn mới được nhập.",
    ),
    ("The parameters of `{command}` must be an object.", "Tham số của `{command}` phải là một đối tượng."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora làm việc trên tài liệu đang mở trong cửa sổ này: `doc` lại chỉ tới một tài liệu khác.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "Lệnh `{command}` gặp lỗi bất ngờ ({error}); tài liệu không bị thay đổi.",
    ),
    ("Commands in one batch: {count} at most.", "Số lệnh trong một lô: tối đa {count}."),
    ("`{command}` failed: {error}", "Lệnh `{command}` thất bại: {error}"),
    (
        "Occurrences of “{text}” in the document: {count} (not {occurrence}).",
        "Số lần xuất hiện của “{text}” trong tài liệu: {count} (không phải {occurrence}).",
    ),
    ("“{text}” is not in the first {pages} pages of the document.", "“{text}” không có trong {pages} trang đầu của tài liệu."),
    ("“{text}” is not in the document.", "“{text}” không có trong tài liệu."),
    ("There is no text to type.", "Không có văn bản nào để nhập."),
    ("There is no page {page}: the last page is {count}.", "Không có trang {page}: trang cuối là trang {count}."),
    ("{name} was not saved: {why}", "Tệp {name} chưa được lưu: {why}"),
    ("Nora has nothing left to undo.", "Nora không còn thay đổi nào để hoàn tác."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Tài liệu đã thay đổi kể từ lần chỉnh sửa cuối của Nora: hãy dùng Hoàn tác trong trình soạn thảo.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Hoàn tác thất bại ở bước {step} trên {count}: {error}"),
    ("New conversation", "Cuộc trò chuyện mới"),
    ("Undo the last change Nora made to the document", "Hoàn tác thay đổi gần nhất của Nora trong tài liệu"),
    ("Ask Nora…", "Hỏi Nora…"),
    ("Send", "Gửi"),
    ("Nora is working…", "Nora đang làm việc…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Hãy nhờ Nora đọc, tóm tắt, điền hoặc chú thích tài liệu của bạn."),
    ("Nora wants to:", "Nora muốn:"),
    ("Refuse", "Từ chối"),
    ("Read the document", "Đọc tài liệu"),
    ("Run {command}", "Chạy {command}"),
    ("Run several commands ({count})", "Chạy nhiều lệnh ({count})"),
    ("Select “{text}”", "Chọn “{text}”"),
    ("Type “{text}”", "Nhập “{text}”"),
    ("Look at page {page}", "Xem trang {page}"),
    ("Save the document", "Lưu tài liệu"),
    ("Undo the last change", "Hoàn tác thay đổi gần nhất"),
    ("Assistant", "Trợ lý"),
    ("Show the Nora assistant pane (experimental)", "Hiện bảng trợ lý Nora (thử nghiệm)"),
    ("Nora assistant", "Trợ lý Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "Lệnh `{command}` bị từ chối: chỉ bạn mới được thêm chữ ký và chữ viết tắt của mình.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "Lệnh `{command}` bị từ chối: thiết lập bảo mật của tài liệu này không cho phép sao chép văn bản.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Thay đổi của Nora đã quá cũ nên không thể hoàn tác toàn bộ: hãy dùng Hoàn tác trong trình soạn thảo.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Dừng"),
    ("Stopped.", "Đã dừng."),
    ("Nora did not answer in time.", "Nora không trả lời kịp thời."),
    ("Nora could not be reached. Check the connection and try again.", "Không thể kết nối tới Nora. Hãy kiểm tra kết nối rồi thử lại."),
    ("Your session has expired. Sign in again.", "Phiên làm việc của bạn đã hết hạn. Hãy đăng nhập lại."),
    ("The server could not answer (error {status}). Try again in a moment.", "Máy chủ không thể phản hồi (lỗi {status}). Hãy thử lại sau ít phút."),
    ("Nora's answer could not be read.", "Không thể đọc câu trả lời của Nora."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Thẻ này đang ở nền: Nora không thể thao tác trong tài liệu."),
    ("Resume", "Tiếp tục"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Chưa có gì được thực hiện trong tài liệu: Nora không thể thao tác kịp thời. Hãy yêu cầu lại nếu bạn vẫn cần.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Không phải tất cả đều được thực hiện: một phần không thể chạy trong tài liệu kịp thời.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Hỏi Nora"),
    ("Nora, the Neoffice assistant", "Nora, trợ lý của Neoffice"),
];
