// //// Neoffice — added file (no upstream equivalent)
//! Japanese words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora はこのバージョンではまだ利用できません"),
    ("Nora is waiting for you to finish what you are doing in the document.", "文書での作業が終わるまで、Nora は待機しています。"),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "このカードには、Neoffice の Nora のチャットで回答してください。NeoPDF はまだこの回答を処理できません。",
    ),
    ("There is no document open for Nora to work in.", "Nora が作業できる文書が開かれていません。"),
    ("`{command}` is refused: Nora may not run it.", "`{command}` は拒否されました: Nora は実行を許可されていません。"),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "`{command}` は NeoPDF では Nora が実行できません: このコマンドは画面からの操作でのみ動作します。",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "`{command}` は複数のツール（{tools}）で使えます: 実行するツールを指定してください。",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` は NeoPDF のコマンドではありません: Nora が実行できるものは command_list に一覧があります。",
    ),
    ("`{command}` is refused: its parameters are nested too deeply.", "`{command}` は拒否されました: パラメーターの入れ子が深すぎます。"),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "`{command}` は拒否されました: `{key}` がコンピューター上のファイルにアクセスしてしまいます。",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "`{command}` は拒否されました: `{key}` はパスワードで、入力できるのはご本人だけです。",
    ),
    ("The parameters of `{command}` must be an object.", "`{command}` のパラメーターはオブジェクトである必要があります。"),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora は、このウィンドウで開いている文書で作業します: `doc` は別の文書を指定しています。",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "`{command}` が予期せず失敗しました（{error}）。文書は変更されていません。",
    ),
    ("Commands in one batch: {count} at most.", "一括実行のコマンド数: 最大 {count}。"),
    ("`{command}` failed: {error}", "`{command}` が失敗しました: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "文書内の「{text}」の出現数: {count}（{occurrence} ではありません）。"),
    ("“{text}” is not in the first {pages} pages of the document.", "「{text}」は文書の最初の {pages} ページにありません。"),
    ("“{text}” is not in the document.", "「{text}」は文書にありません。"),
    ("There is no text to type.", "入力するテキストがありません。"),
    ("There is no page {page}: the last page is {count}.", "ページ {page} は存在しません: 最後のページは {count} ページです。"),
    ("{name} was not saved: {why}", "{name} は保存されませんでした: {why}"),
    ("Nora has nothing left to undo.", "Nora が取り消せる変更はもうありません。"),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Nora の最後の変更の後に文書が変更されています: 代わりにエディターで「取り消し」を使ってください。",
    ),
    ("Undo failed at step {step} of {count}: {error}", "取り消しは {count} 手順中の {step} 番目で失敗しました: {error}"),
    ("New conversation", "新しい会話"),
    ("Undo the last change Nora made to the document", "Nora が文書に行った最後の変更を取り消します"),
    ("Ask Nora…", "Nora に質問…"),
    ("Send", "送信"),
    ("Nora is working…", "Nora が作業中…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Nora に文書の読み取り、要約、入力、注釈付けを頼んでください。"),
    ("Nora wants to:", "Nora が次の操作を求めています:"),
    ("Refuse", "拒否"),
    ("Read the document", "文書を読む"),
    ("Run {command}", "{command} を実行"),
    ("Run several commands ({count})", "複数のコマンドを実行（{count}）"),
    ("Select “{text}”", "「{text}」を選択"),
    ("Type “{text}”", "「{text}」と入力"),
    ("Look at page {page}", "ページ {page} を確認"),
    ("Save the document", "文書を保存"),
    ("Undo the last change", "最後の変更を取り消す"),
    ("Assistant", "アシスタント"),
    ("Show the Nora assistant pane (experimental)", "Nora アシスタントのパネルを表示（実験的）"),
    ("Nora assistant", "Nora アシスタント"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "`{command}` は拒否されました: 署名とイニシャルを追加できるのは本人だけです。",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "`{command}` は拒否されました: この文書のセキュリティ設定により、そのテキストのコピーは禁止されています。",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "Nora の変更は古すぎて全体を取り消せません: 代わりにエディターで「取り消し」を使ってください。",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "停止"),
    ("Stopped.", "停止しました。"),
    ("Nora did not answer in time.", "Nora からの応答が時間内に届きませんでした。"),
    ("Nora could not be reached. Check the connection and try again.", "Nora に接続できませんでした。接続を確認して、もう一度お試しください。"),
    ("Your session has expired. Sign in again.", "セッションの有効期限が切れました。もう一度サインインしてください。"),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "サーバーから応答がありませんでした（エラー {status}）。しばらくしてから再度お試しください。",
    ),
    ("Nora's answer could not be read.", "Nora の回答を読み取れませんでした。"),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    (
        "This tab was in the background: Nora could not act in the document.",
        "このタブはバックグラウンドにあったため、Nora は文書内で操作できませんでした。",
    ),
    ("Resume", "再開"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "文書内では何も実行されませんでした。Nora は時間内に操作できませんでした。必要であれば、もう一度依頼してください。",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "一部だけが実行されました。残りは時間内に文書内で実行できませんでした。",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Noraに質問"),
    ("Nora, the Neoffice assistant", "Nora、Neoffice のアシスタント"),
];
