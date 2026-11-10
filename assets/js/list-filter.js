// Señala en los filtros de listado (`ListFilter`) que se aplican con botón los campos cuyo valor ya
// no coincide con el del filtro aplicado, para que el usuario sepa que el listado todavía no
// corresponde a lo que muestra el formulario. Sólo pone y quita clases (`list-filter-changed` en el
// contenedor de cada campo, `list-filter-pending` en el formulario) y el texto del aviso para
// lectores de pantalla; el aspecto lo da cada tema. Sin dependencias.
//
// Actúa sólo en los formularios con el aviso `.list-filter-status`, que `ListFilter` no pinta
// cuando los filtros se envían solos.

(function () {
    "use strict";

    var FIELDS =
        'input:not([type="hidden"]):not([type="submit"])' +
        ':not([type="button"]):not([type="reset"]), select, textarea';

    // Opción que estaba elegida al aplicar el filtro. Un desplegable simple sin ninguna opción
    // marcada en el HTML muestra la primera.
    function wasSelected(select, option, index) {
        if (option.defaultSelected) {
            return true;
        }
        if (select.multiple) {
            return false;
        }
        for (var i = 0; i < select.options.length; i++) {
            if (select.options[i].defaultSelected) {
                return false;
            }
        }
        return index === 0;
    }

    // Indica si el campo tiene un valor distinto del que tenía al aplicar el filtro.
    function isChanged(field) {
        if (field.type === "checkbox" || field.type === "radio") {
            return field.checked !== field.defaultChecked;
        }
        if (field.tagName === "SELECT") {
            for (var i = 0; i < field.options.length; i++) {
                if (field.options[i].selected !== wasSelected(field, field.options[i], i)) {
                    return true;
                }
            }
            return false;
        }
        return field.value !== field.defaultValue;
    }

    // Toma los valores actuales como los del filtro aplicado. Hace falta cuando sólo se actualiza
    // el listado y el formulario no se vuelve a pintar.
    function commit(field) {
        if (field.type === "checkbox" || field.type === "radio") {
            field.defaultChecked = field.checked;
        } else if (field.tagName === "SELECT") {
            for (var i = 0; i < field.options.length; i++) {
                field.options[i].defaultSelected = field.options[i].selected;
            }
        } else {
            field.defaultValue = field.value;
        }
    }

    function refresh(form) {
        var changed = [];
        form.querySelectorAll(FIELDS).forEach(function (field) {
            var container = field.closest(".form-field") || field;
            if (changed.indexOf(container) === -1 && isChanged(field)) {
                changed.push(container);
            }
        });
        form.querySelectorAll(".list-filter-changed").forEach(function (container) {
            if (changed.indexOf(container) === -1) {
                container.classList.remove("list-filter-changed");
            }
        });
        changed.forEach(function (container) {
            container.classList.add("list-filter-changed");
        });

        var pending = changed.length > 0;
        form.classList.toggle("list-filter-pending", pending);
        // Sólo se cambia el texto al empezar o terminar los cambios pendientes, para que el lector
        // de pantalla lo anuncie una vez y no con cada pulsación.
        var status = form.querySelector(".list-filter-status");
        var text = pending ? status.getAttribute("data-pending") || "" : "";
        if (status.textContent !== text) {
            status.textContent = text;
        }
    }

    function filterOf(element) {
        var form = element instanceof Element ? element.closest("form.list-filter") : null;
        return form && form.querySelector(".list-filter-status") ? form : null;
    }

    function onChange(event) {
        var form = filterOf(event.target);
        if (form) {
            refresh(form);
        }
    }

    document.addEventListener("input", onChange);
    document.addEventListener("change", onChange);

    document.addEventListener("submit", function (event) {
        var form = filterOf(event.target);
        if (form) {
            form.querySelectorAll(FIELDS).forEach(commit);
            refresh(form);
        }
    });
})();
