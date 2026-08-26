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

// Cada `TopLinkDisclosureMenu` es independiente: al hacer clic fuera sólo cierra su propio menú
// (ver `_handleClick()` en accessible-menu), así que abrir un desplegable no cierra otro que ya
// estuviera abierto en otra parte de la página. Se corrige aquí escuchando `accessibleMenuExpand`
// -evento que la librería emite en cada apertura- y cerrando el resto de menús de nivel superior
// ya registrados, vía `window.AccessibleMenu.menus`, el registro global que la propia librería
// mantiene con todas las instancias (incluye las de `basic.menu.init.js`).
document.addEventListener("accessibleMenuExpand", function (event) {
    var openedToggle = event.detail.toggle;
    var menus = (window.AccessibleMenu && window.AccessibleMenu.menus) || {};
    Object.values(menus).forEach(function (menu) {
        var controller = menu.elements.controller;
        if (controller && controller !== openedToggle) {
            controller.close();
        }
    });
});

// El cierre "al hacer clic fuera" que trae la librería no funciona para un `controllerElement` de
// nivel superior como el nuestro: en `toggleToggle()` (accessible-menu), al abrir, hace
// `menu.focusState = "self"` y a continuación `toggle.elements.controlledMenu.focusState =
// "none"` -pensado para cuando el toggle abre un submenú hijo distinto del menú que lo contiene-,
// pero aquí `menu` y `toggle.elements.controlledMenu` son el mismo objeto: la segunda asignación
// deshace la primera. El cierre nativo de la librería depende de `focusState !== "none"`, así que
// nunca se dispara aunque el menú esté abierto (`isOpen` sí queda correcto). Se sustituye aquí por
// una comprobación propia basada sólo en `isOpen` y en la posición del clic.
document.addEventListener("click", function (event) {
    var menus = (window.AccessibleMenu && window.AccessibleMenu.menus) || {};
    Object.values(menus).forEach(function (menu) {
        var controller = menu.elements.controller;
        if (
            controller &&
            controller.isOpen &&
            !menu.dom.menu.contains(event.target) &&
            !controller.dom.toggle.contains(event.target)
        ) {
            controller.close();
        }
    });
});
