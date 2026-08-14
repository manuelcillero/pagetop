// Conecta cada `Dropdown` (standalone o embebido en un `Nav`) con `accessible-menu` (ver
// basic.menu.min.js), que añade los roles ARIA, el `aria-expanded` y la navegación por teclado
// sobre el marcado ya generado en el servidor. Selecciona por clase, no por etiqueta, porque el
// disparador es un `<button>` en un `Dropdown` independiente y un `<a>` cuando cuelga de un
// `nav::Item::dropdown()`.
document.querySelectorAll(".dropdown > .dropdown-toggle").forEach(function (toggle) {
    var container = toggle.parentElement;
    var menu = container.querySelector(":scope > .dropdown-menu");
    if (!menu) {
        return;
    }

    new TopLinkDisclosureMenu({
        menuElement: menu,
        containerElement: container,
        controllerElement: toggle,
        // `Header`/`Divider`/`Label` no son interactivos: se excluyen de `menuItemSelector`
        // (si no, la librería asume que todo `<li>` tiene un enlace y lanza un TypeError al
        // intentar añadirle listeners de foco).
        menuItemSelector: "li:has(> a.dropdown-item, > button.dropdown-item)",
        menuLinkSelector: "a.dropdown-item, button.dropdown-item",
        // La navegación con flechas/Home/End es opcional en la librería (`false` por defecto);
        // se activa para igualar el comportamiento habitual de un menú desplegable.
        optionalKeySupport: true,
    });
});
