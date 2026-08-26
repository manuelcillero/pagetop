// Abre y cierra los componentes `Dialog` de Basic (elemento `<dialog>` nativo) interpretando el
// vocabulario de atributos `data-dialog-*`, común a todos los temas: Bootsier traduce estos mismos
// nombres a los suyos propios (`data-bs-*`) en `setup()` para que el JS de Bootstrap los reconozca,
// así el marcado de un disparador o de un botón de cierre es idéntico en los dos temas. Sin
// dependencias: ni htmx, ni Bootstrap, ni ninguna otra librería.

// Abre el diálogo referenciado por `data-dialog-target` al pulsar un elemento con
// `data-dialog-toggle="modal"`.
document.addEventListener("click", function (event) {
    var toggle = event.target.closest('[data-dialog-toggle="modal"]');
    if (!toggle) {
        return;
    }
    var target = toggle.getAttribute("data-dialog-target");
    var dialog = target && document.querySelector(target);
    if (!(dialog instanceof HTMLDialogElement)) {
        return;
    }
    event.preventDefault();
    dialog.showModal();
});

// Cierra el `<dialog>` ascendente más cercano al pulsar un elemento con
// `data-dialog-dismiss="modal"`.
document.addEventListener("click", function (event) {
    var dismiss = event.target.closest('[data-dialog-dismiss="modal"]');
    if (!dismiss) {
        return;
    }
    var dialog = dismiss.closest("dialog");
    if (dialog) {
        event.preventDefault();
        dialog.close();
    }
});

// Cierra el diálogo al pulsar sobre el fondo (`::backdrop`): un clic ahí llega con el propio
// `<dialog>` como `event.target`, porque el fondo no forma parte de su caja de contenido.
document.querySelectorAll("dialog.dialog").forEach(function (dialog) {
    dialog.addEventListener("click", function (event) {
        if (event.target === dialog) {
            dialog.close();
        }
    });
});
