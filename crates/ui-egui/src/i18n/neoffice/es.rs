// //// Neoffice — added file (no upstream equivalent)
//! Spanish words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora todavía no está disponible en esta versión"),
    ("Nora is waiting for you to finish what you are doing in the document.", "Nora espera a que termines lo que estás haciendo en el documento."),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Responda a esta tarjeta en el chat de Nora en Neoffice: NeoPDF todavía no puede procesarla.",
    ),
    ("There is no document open for Nora to work in.", "No hay ningún documento abierto en el que Nora pueda trabajar."),
    ("`{command}` is refused: Nora may not run it.", "El comando `{command}` ha sido rechazado: Nora no tiene permiso para ejecutarlo."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "El comando `{command}` no puede ser ejecutado por Nora en NeoPDF: solo funciona a través de la interfaz.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "El comando `{command}` está en varias herramientas ({tools}): indica cuál debe ejecutarse.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` no es un comando de NeoPDF: command_list enumera los que Nora puede ejecutar.",
    ),
    (
        "`{command}` is refused: its parameters are nested too deeply.",
        "El comando `{command}` ha sido rechazado: sus parámetros están anidados demasiado a fondo.",
    ),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "El comando `{command}` ha sido rechazado: `{key}` accedería a un archivo del ordenador.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "El comando `{command}` ha sido rechazado: `{key}` es una contraseña, que solo tú introduces.",
    ),
    ("The parameters of `{command}` must be an object.", "Los parámetros de `{command}` deben ser un objeto."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora trabaja en el documento abierto en esta ventana: `doc` indica otro documento.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "El comando `{command}` ha fallado de forma inesperada ({error}); el documento no se ha modificado.",
    ),
    ("Commands in one batch: {count} at most.", "Comandos en un solo lote: {count} como máximo."),
    ("`{command}` failed: {error}", "El comando `{command}` ha fallado: {error}"),
    ("Occurrences of “{text}” in the document: {count} (not {occurrence}).", "Apariciones de «{text}» en el documento: {count} (no {occurrence})."),
    ("“{text}” is not in the first {pages} pages of the document.", "«{text}» no aparece en las primeras {pages} páginas del documento."),
    ("“{text}” is not in the document.", "«{text}» no aparece en el documento."),
    ("There is no text to type.", "No hay ningún texto que escribir."),
    ("There is no page {page}: the last page is {count}.", "No existe la página {page}: la última es la página {count}."),
    ("{name} was not saved: {why}", "El archivo {name} no se ha guardado: {why}"),
    ("Nora has nothing left to undo.", "Nora ya no tiene nada que deshacer."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "El documento ha cambiado desde el último cambio de Nora: usa en su lugar Deshacer en el editor.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "Deshacer ha fallado en el paso {step} de {count}: {error}"),
    ("New conversation", "Nueva conversación"),
    ("Undo the last change Nora made to the document", "Deshacer el último cambio que Nora hizo en el documento"),
    ("Ask Nora…", "Pregunta a Nora…"),
    ("Send", "Enviar"),
    ("Nora is working…", "Nora está trabajando…"),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Pide a Nora que lea, resuma, rellene o anote tu documento."),
    ("Nora wants to:", "Nora quiere:"),
    ("Refuse", "Rechazar"),
    ("Read the document", "Leer el documento"),
    ("Run {command}", "Ejecutar {command}"),
    ("Run several commands ({count})", "Ejecutar varios comandos ({count})"),
    ("Select “{text}”", "Seleccionar «{text}»"),
    ("Type “{text}”", "Escribir «{text}»"),
    ("Look at page {page}", "Ver la página {page}"),
    ("Save the document", "Guardar el documento"),
    ("Undo the last change", "Deshacer el último cambio"),
    ("Assistant", "Asistente"),
    ("Show the Nora assistant pane (experimental)", "Mostrar el panel del asistente Nora (experimental)"),
    ("Nora assistant", "Asistente Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "El comando `{command}` ha sido rechazado: solo tú puedes añadir tu firma y tus iniciales.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "El comando `{command}` ha sido rechazado: la configuración de seguridad de este documento prohíbe copiar su texto.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "El cambio de Nora es demasiado antiguo para deshacerlo por completo: usa en su lugar Deshacer en el editor.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Detener"),
    ("Stopped.", "Detenido."),
    ("Nora did not answer in time.", "Nora no respondió a tiempo."),
    ("Nora could not be reached. Check the connection and try again.", "No se pudo contactar con Nora. Compruebe la conexión y vuelva a intentarlo."),
    ("Your session has expired. Sign in again.", "Su sesión ha caducado. Inicie sesión de nuevo."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "El servidor no pudo responder (error {status}). Inténtelo de nuevo en unos instantes.",
    ),
    ("Nora's answer could not be read.", "No se pudo leer la respuesta de Nora."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    (
        "This tab was in the background: Nora could not act in the document.",
        "Esta pestaña estaba en segundo plano: Nora no pudo actuar en el documento.",
    ),
    ("Resume", "Reanudar"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "No se hizo nada en el documento: Nora no pudo actuar en él a tiempo. Vuelve a pedirlo si todavía lo necesitas.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "No se hizo todo: una parte no se pudo ejecutar en el documento a tiempo.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Preguntar a Nora"),
    ("Nora, the Neoffice assistant", "Nora, la asistente de Neoffice"),
];
