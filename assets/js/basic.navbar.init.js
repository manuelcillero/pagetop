// Conecta cada botón `.navbar-toggle` con el contenido que controla (`aria-controls`), alternando
// `aria-expanded` en el botón y la clase `.show` en el contenido. A diferencia de `Dropdown` no usa
// `accessible-menu`: el contenido de una barra de navegación no es una única lista de elementos
// (puede llevar varios `Nav`, texto, lo que sea), así que no encaja con el modelo de menú de esa
// librería (ver `base/component/navbar/`).
document.querySelectorAll(".navbar-toggle").forEach(function (toggle) {
    var id = toggle.getAttribute("aria-controls");
    var content = id && document.getElementById(id);
    if (!content) {
        return;
    }

    toggle.addEventListener("click", function () {
        var expanded = toggle.getAttribute("aria-expanded") === "true";
        toggle.setAttribute("aria-expanded", String(!expanded));
        content.classList.toggle("show", !expanded);
    });
});
