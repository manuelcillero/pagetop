// Confirma en los diálogos de `hx_confirm` (`dialog()` y `dialog_delete()`) las peticiones de los
// botones que preparan `hx_confirm::confirm()` y `confirm_delete()`, en vez de con el `confirm()`
// del navegador.
//
// El botón abre su diálogo con el vocabulario `data-dialog-*` de PageTop, como cualquier otro
// `Button`, así que lo abre el tema activo con su propio mecanismo. Este script sólo copia la
// pregunta de `hx-confirm` en el diálogo, retiene la petición y la reanuda tal cual (con su
// `hx-target`, `hx-swap`...) si se pulsa el botón de aceptar de ese mismo diálogo. No conoce ningún
// tema: localiza cada diálogo por su `id` y los botones por atributos que los temas no renombran.
(function () {
    "use strict";

    // El botón lleva en `data-hx-confirm` el nombre de su diálogo, `hx-confirm-<nombre>`, cuya
    // pregunta es `hx-confirm-<nombre>-question`.
    var PREFIX = "hx-confirm-";
    var MARK = "data-hx-confirm";

    // Petición retenida (el `detail` de `htmx:confirm`) y su diálogo, sólo la del último botón
    // pulsado.
    var pending = null;
    // Petición confirmada, para devolver el foco al listado cuando su botón desaparezca.
    var confirmed = null;

    // Cada pulsación de un botón marcado descarta la petición retenida antes de que HTMX pida la
    // suya: si el diálogo se abriera sin llegar `htmx:confirm`, aceptar no reenviaría la petición
    // de otro botón.
    document.addEventListener("click", function (event) {
        if (event.target instanceof Element && event.target.closest("[" + MARK + "]")) {
            pending = null;
        }
    }, true);

    // En fase de captura, antes que cualquier otro manejador de `htmx:confirm` (como el modal
    // genérico de un tema), que puede apartarse al verlo ya atendido (`defaultPrevented`).
    document.addEventListener("htmx:confirm", function (event) {
        var detail = event.detail;
        var name = detail.elt.getAttribute(MARK);
        var dialog = name && document.getElementById(PREFIX + name);
        var question = dialog && document.getElementById(PREFIX + name + "-question");
        // Sin el diálogo en la página, HTMX sigue como con cualquier `hx-confirm`. Con él, se
        // retiene aunque la pregunta esté vacía: HTMX enviaría entonces la petición sin confirmar.
        if (!question) {
            return;
        }
        event.preventDefault();
        question.textContent = detail.question || "";
        pending = { detail: detail, dialog: dialog };
    }, true);

    document.addEventListener("click", function (event) {
        if (!(event.target instanceof Element) || !pending) {
            return;
        }
        var accept = event.target.closest("[data-hx-confirm-accept]");
        if (!accept || !pending.dialog.contains(accept)) {
            return;
        }
        confirmed = pending;
        pending = null;
        confirmed.detail.issueRequest(true);
    });

    // La respuesta suele sustituir el listado y, con él, el botón que tenía el foco: se lleva al
    // contenido nuevo para que el teclado y los lectores de pantalla sigan en el listado.
    document.addEventListener("htmx:afterSettle", function (event) {
        // HTMX lo lanza en cada nodo nuevo, también en los de texto: se espera a un elemento.
        var target = event.target;
        var request = event.detail.requestConfig;
        if (!confirmed || !request || request.elt !== confirmed.detail.elt) {
            return;
        }
        if (!(target instanceof HTMLElement)) {
            return;
        }
        var dialog = confirmed.dialog;
        confirmed = null;
        // El foco se ha perdido si quedó en `<body>`, en un elemento que ya no está o en el propio
        // diálogo, que un tema puede seguir cerrando con una transición cuando llega la respuesta.
        var active = document.activeElement;
        if (active && active !== document.body && active.isConnected && !dialog.contains(active)) {
            return;
        }
        if (!target.hasAttribute("tabindex")) {
            target.setAttribute("tabindex", "-1");
        }
        target.focus();
    });
}());
