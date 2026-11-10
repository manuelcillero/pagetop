// Anuncia a los lectores de pantalla el resumen de un listado (`ListView`, atributo
// `data-summary`, p. ej. "23 resultados") cuando se actualiza sin recargar la página, al filtrar,
// ordenar o cambiar de página. Un lector de pantalla no suele anunciar una zona `aria-live` que se
// inserta de golpe, así que el aviso usa una única zona creada al cargar la página, y el listado
// nuevo sólo le pasa su texto. Al cargar la página no anuncia nada. Sin dependencias: vale para
// HTMX o para cualquier otra forma de actualizar sólo una parte de la página.

(function () {
    "use strict";

    var SELECTOR = ".list-view[data-summary]";
    var REGION_ID = "list-view-status";

    // La zona se busca por su identificador cada vez: si se restaura una página del historial
    // sustituyendo el contenido del `<body>` (como hace HTMX al volver atrás), la zona creada al
    // cargar desaparece y queda la copia restaurada.
    function region() {
        var element = document.getElementById(REGION_ID);
        if (!element) {
            element = document.createElement("div");
            element.id = REGION_ID;
            element.className = "visually-hidden";
            element.setAttribute("role", "status");
            document.body.appendChild(element);
        }
        return element;
    }
    region();

    function announce(text) {
        var element = region();
        // Se vacía primero para que se anuncie también un texto igual al anterior.
        element.textContent = "";
        setTimeout(function () {
            element.textContent = text;
        }, 50);
    }

    new MutationObserver(function (mutations) {
        mutations.forEach(function (mutation) {
            mutation.addedNodes.forEach(function (node) {
                if (!(node instanceof Element)) {
                    return;
                }
                var list = node.matches(SELECTOR) ? node : node.querySelector(SELECTOR);
                if (list) {
                    announce(list.getAttribute("data-summary"));
                }
            });
        });
    }).observe(document.body, { childList: true, subtree: true });
})();
