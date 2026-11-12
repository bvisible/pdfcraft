// //// Neoffice — added file (no upstream equivalent)
//! French words of the Nora assistant pane, its switch in Preferences and its command: English source, then translation
//! (written by a language model for the fork, not separately native-reviewed). Read by `super::plain`.

pub(super) const ROWS: &[(&str, &str)] = &[
    // the Nora assistant pane (nora_pane.rs), its switch in Preferences and its command `ui.nora`.
    ("Nora isn't available in this version yet", "Nora n'est pas encore disponible dans cette version"),
    (
        "Nora is waiting for you to finish what you are doing in the document.",
        "Nora attend que vous ayez terminé ce que vous faites dans le document.",
    ),
    (
        "Answer this card in Nora's chat in Neoffice: NeoPDF does not take it yet.",
        "Répondez à cette carte dans le chat de Nora dans Neoffice : NeoPDF ne la prend pas encore en charge.",
    ),
    ("There is no document open for Nora to work in.", "Aucun document n'est ouvert dans lequel Nora puisse travailler."),
    ("`{command}` is refused: Nora may not run it.", "La commande `{command}` est refusée : Nora n'a pas le droit de l'exécuter."),
    (
        "`{command}` can't be run by Nora in NeoPDF: it works only through the interface.",
        "La commande `{command}` ne peut pas être exécutée par Nora dans NeoPDF : elle ne fonctionne que par l'interface.",
    ),
    (
        "`{command}` is played by several tools ({tools}): name the one to run.",
        "La commande `{command}` est proposée par plusieurs outils ({tools}) : indiquez celui à exécuter.",
    ),
    (
        "`{command}` is not a command of NeoPDF: command_list lists the ones Nora can run.",
        "`{command}` n'est pas une commande de NeoPDF : command_list liste celles que Nora peut exécuter.",
    ),
    (
        "`{command}` is refused: its parameters are nested too deeply.",
        "La commande `{command}` est refusée : ses paramètres sont imbriqués trop profondément.",
    ),
    (
        "`{command}` is refused: `{key}` would reach a file of the computer.",
        "La commande `{command}` est refusée : `{key}` atteindrait un fichier de l'ordinateur.",
    ),
    (
        "`{command}` is refused: `{key}` is a password, which only you enter.",
        "La commande `{command}` est refusée : `{key}` est un mot de passe, que vous seul saisissez.",
    ),
    ("The parameters of `{command}` must be an object.", "Les paramètres de `{command}` doivent être un objet."),
    (
        "Nora works in the document open in this window: `doc` names another one.",
        "Nora travaille dans le document ouvert dans cette fenêtre : `doc` désigne un autre document.",
    ),
    (
        "`{command}` failed unexpectedly ({error}); the document was not changed.",
        "La commande `{command}` a échoué de façon inattendue ({error}) : le document n'a pas été modifié.",
    ),
    ("Commands in one batch: {count} at most.", "Commandes dans un même lot : {count} au maximum."),
    ("`{command}` failed: {error}", "La commande `{command}` a échoué : {error}"),
    (
        "Occurrences of “{text}” in the document: {count} (not {occurrence}).",
        "Occurrences de « {text} » dans le document : {count} (et non {occurrence}).",
    ),
    ("“{text}” is not in the first {pages} pages of the document.", "« {text} » ne figure pas dans les {pages} premières pages du document."),
    ("“{text}” is not in the document.", "« {text} » ne figure pas dans le document."),
    ("There is no text to type.", "Il n'y a aucun texte à saisir."),
    ("There is no page {page}: the last page is {count}.", "Il n'y a pas de page {page} : la dernière est la page {count}."),
    ("{name} was not saved: {why}", "Le fichier {name} n'a pas été enregistré : {why}"),
    ("Nora has nothing left to undo.", "Il n'y a plus aucune modification de Nora à annuler."),
    (
        "The document has changed since Nora's last change: use Undo in the editor instead.",
        "Le document a été modifié depuis la dernière modification de Nora : utilisez plutôt la fonction Annuler de l'éditeur.",
    ),
    ("Undo failed at step {step} of {count}: {error}", "L'annulation a échoué à l'étape {step} sur {count} : {error}"),
    ("New conversation", "Nouvelle conversation"),
    ("Undo the last change Nora made to the document", "Annuler la dernière modification faite par Nora dans le document"),
    ("Ask Nora…", "Demander à Nora…"),
    ("Send", "Envoyer"),
    ("Nora is working…", "Nora travaille…"),
    // the Nora pane of 05.11.2026 (craft-libs 7bc770d): what Nora is doing, the ready requests, the microphone.
    ("Nora is thinking…", "Nora réfléchit…"),
    ("Nora is reading the PDF…", "Nora lit le PDF…"),
    ("Nora is looking through the PDF…", "Nora parcourt le PDF…"),
    ("Nora is finishing up…", "Nora termine…"),
    ("How can I help?", "Comment puis-je vous aider ?"),
    ("Undo this change", "Annuler cette modification"),
    ("Summarise this PDF", "Résumer ce PDF"),
    ("List the key dates and amounts", "Lister les dates et montants clés"),
    ("What is this document about?", "De quoi parle ce document ?"),
    ("Dictate", "Dicter"),
    ("Stop and keep what you said", "Arrêter et garder ce que vous avez dit"),
    ("Writing down what you said…", "Nora écrit ce que vous avez dit…"),
    ("I did not hear anything. Try again, a little closer to the microphone.", "Je n'ai rien entendu. Réessayez, un peu plus près du micro."),
    ("The microphone is blocked. Allow it for this page, then try again.", "Le micro est bloqué. Autorisez-le pour cette page, puis réessayez."),
    ("No microphone found on this computer.", "Aucun micro n'a été trouvé sur cet ordinateur."),
    ("The dictation did not work. Try again, or type your request.", "La dictée n'a pas fonctionné. Réessayez ou écrivez votre demande."),
    ("Ask Nora to read, summarize, fill in or annotate your document.", "Demandez à Nora de lire, résumer, remplir ou annoter votre document."),
    ("Nora wants to:", "Nora souhaite :"),
    ("Refuse", "Refuser"),
    ("Read the document", "Lire le document"),
    ("Run {command}", "Exécuter {command}"),
    ("Run several commands ({count})", "Exécuter plusieurs commandes ({count})"),
    ("Select “{text}”", "Sélectionner « {text} »"),
    ("Type “{text}”", "Saisir « {text} »"),
    ("Look at page {page}", "Consulter la page {page}"),
    ("Save the document", "Enregistrer le document"),
    ("Undo the last change", "Annuler la dernière modification"),
    ("Assistant", "Assistant"),
    ("Show the Nora assistant pane (experimental)", "Afficher le volet de l'assistant Nora (expérimental)"),
    ("Nora assistant", "Assistant Nora"),
    // the Nora assistant pane (nora_pane.rs): what it refuses, and its Undo of an old change.
    (
        "`{command}` is refused: your signature and initials are yours to add.",
        "La commande `{command}` est refusée : c'est à vous seul d'ajouter votre signature et vos initiales.",
    ),
    (
        "`{command}` is refused: this document's security forbids copying its text.",
        "La commande `{command}` est refusée : la protection de ce document interdit de copier son texte.",
    ),
    (
        "Nora's change is too old to be taken back whole: use Undo in the editor instead.",
        "La modification de Nora est trop ancienne pour être annulée intégralement : utilisez plutôt la fonction Annuler de l'éditeur.",
    ),
    // the Nora assistant pane (nora_pane.rs): its Stop button, and its words for a failure of the connection to Nora (craft-libs adbcef6).
    ("Stop", "Arrêter"),
    ("Stopped.", "Arrêté."),
    ("Nora did not answer in time.", "Nora n'a pas répondu à temps."),
    ("Nora could not be reached. Check the connection and try again.", "Nora n'a pas pu être jointe. Vérifiez la connexion et réessayez."),
    ("Your session has expired. Sign in again.", "Votre session a expiré. Reconnectez-vous."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Le serveur n'a pas pu répondre (erreur {status}). Réessayez dans un instant.",
    ),
    ("Nora's answer could not be read.", "La réponse de Nora n'a pas pu être lue."),
    // the Nora assistant pane (nora_pane.rs): a request a tab in the background kept from acting, and a reply that could not act in time (craft-libs 620d408).
    (
        "This tab was in the background: Nora could not act in the document.",
        "Cet onglet était en arrière-plan : Nora n'a pas pu agir dans le document.",
    ),
    ("Resume", "Reprendre"),
    (
        "Nothing was done in the document: Nora could not act in it in time. Ask again if you still need it.",
        "Rien n'a été fait dans le document : Nora n'a pas pu agir à temps. Redemandez-le si vous en avez encore besoin.",
    ),
    (
        "Not all of this was done: part of it could not run in the document in time.",
        "Tout n'a pas été fait : une partie n'a pas pu être exécutée dans le document à temps.",
    ),
    // the Nora assistant pane (nora_pane.rs): Nora's button in the top bar, which opens and closes it (craft-libs launcher).
    ("Ask Nora", "Demander à Nora"),
    ("Nora, the Neoffice assistant", "Nora, l’assistante Neoffice"),
];
