// //// Neoffice — added file (no upstream equivalent)
//! Brazilian Portuguese words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora ainda não está disponível nesta versão"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora está esperando você terminar o que está fazendo no documento."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Responda a este cartão no chat da Nora no Neoffice: o NeoPDF ainda não consegue processá-lo.",
    ),
    ("There is no document open for Nora to work in.", "Não há nenhum documento aberto para Nora trabalhar."),
    ("`{command}` is refused: Nora may not run it.", "O comando `{command}` foi recusado: Nora não tem permissão para executá-lo."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "O comando `{command}` não pode ser executado por Nora no NeoPDF: ele funciona apenas pela interface.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "O comando `{command}` existe em várias ferramentas ({tools}): indique qual ferramenta executar.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` não é um comando do NeoPDF: command_list lista os que Nora pode executar.",
    ),
    (
        "`{command}` is refused: its parameters are nested too deeply.",
        "O comando `{command}` foi recusado: seus parâmetros estão aninhados em profundidade excessiva.",
    ),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "O comando `{command}` foi recusado: `{key}` acessaria um arquivo do computador.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "O comando `{command}` foi recusado: `{key}` é uma senha, que só você digita.",
    ),
    ("The parameters of `{command}` must be an object.", "Os parâmetros de `{command}` devem ser um objeto."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora trabalha no documento aberto nesta janela: `doc` indica outro documento.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "O comando `{command}` falhou inesperadamente ({error}); o documento não foi alterado.",
    ),
    ("Commands in one batch: {count} at most.", "Comandos em um único lote: no máximo {count}."),
    ("`{command}` failed: {error}", "O comando `{command}` falhou: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Ocorrências de “{text}” no documento: {count} (não {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "“{text}” não está nas primeiras {pages} páginas do documento."),
    ("“{text}” is not in the document.", "“{text}” não está no documento."),
    ("There is no text to type.", "Não há nenhum texto para digitar."),
    ("There is no page {page}: the last page is {count}.", "Não existe a página {page}: a última é a página {count}."),
    ("{name} was not saved: {why}", "O arquivo {name} não foi salvo: {why}"),
    ("Nora has nothing left to undo.", "Nora não tem mais nada para desfazer."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "O documento mudou desde a última alteração de Nora: use Desfazer no editor, em vez disso.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Desfazer falhou na etapa {step} de {count}: {error}"),
    ("New conversation", "Nova conversa"),
    ("Undo the last change Nora made to the document", "Desfazer a última alteração feita por Nora no documento"),
    ("Ask Nora…", "Pergunte a Nora…"),
    ("Send", "Enviar"),
    ("Nora is working…", "Nora está trabalhando…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Peça a Nora que leia, resuma, preencha ou anote o seu documento."),
    ("Nora wants to:", "Nora quer:"),
    ("Refuse", "Recusar"),
    ("Read the document", "Ler o documento"),
    ("Run {command}", "Executar {command}"),
    ("Run several commands ({count})", "Executar vários comandos ({count})"),
    ("Select “{text}”", "Selecionar “{text}”"),
    ("Type “{text}”", "Digitar “{text}”"),
    ("Look at page {page}", "Ver a página {page}"),
    ("Save the document", "Salvar o documento"),
    ("Undo the last change", "Desfazer a última alteração"),
    ("Assistant", "Assistente"),
    ("Show the Nora assistant pane (experimental)", "Mostrar o painel do assistente Nora (experimental)"),
    ("Nora assistant", "Assistente Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "O comando `{command}` foi recusado: somente você pode adicionar sua assinatura e suas iniciais.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "O comando `{command}` foi recusado: as configurações de segurança deste documento proíbem copiar o texto dele.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "A alteração de Nora é antiga demais para ser desfeita por inteiro: use Desfazer no editor, em vez disso.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Parar"),
    ("Stopped.", "Interrompido."),
    ("Nora did not answer in time.", "A Nora não respondeu a tempo."),
    ("Nora could not be reached. Check the connection and try again.", "Não foi possível conectar à Nora. Verifique a conexão e tente novamente."),
    ("Your session has expired. Sign in again.", "Sua sessão expirou. Entre novamente."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "O servidor não conseguiu responder (erro {status}). Tente novamente em instantes.",
    ),
    ("Nora's answer could not be read.", "Não foi possível ler a resposta da Nora."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    ("This tab was in the background: Nora could not act in the document.", "Esta aba estava em segundo plano: Nora não pôde agir no documento."),
    ("Resume", "Retomar"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Nada foi feito no documento: Nora não pôde agir nele a tempo. Pergunte de novo se ainda precisar.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Nem tudo foi feito: parte disso não pôde ser executada no documento a tempo.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Perguntar à Nora"),
    ("Nora, the Neoffice assistant", "Nora, a assistente Neoffice"),
];
