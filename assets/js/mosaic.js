// Reparte los bloques de cada mosaico (`Mosaic`) colocando cada uno en la columna más corta.
//
// Marca el mosaico con `mosaic-js`, que lo convierte en una rejilla (ver `mosaic.scss`): la rejilla
// decide cuántas columnas hay y su ancho, y este script coloca cada bloque en la primera fila de su
// columna, bajándolo con `margin-top` hasta debajo del anterior de esa columna. Se mide primero y
// se escribe después. Sigue el algoritmo de colocación de `display: grid-lanes` (CSS Grid Layout
// Level 3), el mosaico nativo que todavía no admiten todos los navegadores:
//
// - Los bloques fijados a una columna (`--mosaic-column`, `mosaic::Item::with_column()`) se
//   colocan primero, en su orden, en lo alto de su columna; el resto se reparte a su alrededor.
// - Cada uno de los demás, en orden, va al tramo de columnas que lo deja más arriba (uno de varias
//   con `--mosaic-span`, un número o `full` para todas, sin pasar nunca de las que hay). Los tramos
//   que quedan como mucho a `--mosaic-tolerance` del más alto cuentan como empatados, y entre ellos
//   gana el primero a partir del bloque anterior (o el primero, si no hay ninguno después). Así un
//   bloque no salta a otra columna por unos píxeles, y el orden de lectura se parece al de
//   colocación.
//
// No usa una rejilla de filas pequeñas con cada bloque ocupando tantas como mide, la técnica
// habitual de colocación sin JavaScript: Firefox no admite más de 10000 líneas por rejilla y los
// bloques de un mosaico alto acababan superpuestos.
//
// Se vuelve a repartir cuando cambia el tamaño de algún bloque (al cambiar el ancho de las
// columnas, al cargar una imagen...), cuando cambia el ancho de la ventana (puede activar un punto
// de corte) y cuando se añaden o quitan bloques, y tras cada reparto el mosaico emite el evento
// `mosaic:layout`. Sin dependencias: también reparte los mosaicos que se insertan después de cargar
// la página, o que restaura HTMX al volver atrás (llegan ya marcados, por eso no basta la clase
// para saber si un mosaico está atendido).

(function () {
    "use strict";

    var mosaics = new WeakSet();

    var resizer = new ResizeObserver(function (entries) {
        var pending = new Set();
        entries.forEach(function (entry) {
            var mosaic = entry.target.parentElement;
            if (mosaic && mosaics.has(mosaic)) {
                pending.add(mosaic);
            }
        });
        pending.forEach(layout);
    });

    // Entero de una propiedad CSS del bloque, o `fallback` si no es un número.
    function integer(style, name, fallback) {
        var value = parseInt(style.getPropertyValue(name), 10);
        return isNaN(value) ? fallback : value;
    }

    function layout(mosaic) {
        // Primero se quita la colocación anterior: el valor calculado de `grid-template-columns`
        // incluye las columnas implícitas, y un bloque en una columna que ya no existe (o todos en
        // la primera fila) crearía alguna y falsearía la cuenta. Sin ella, la rejilla los coloca en
        // su orden natural y sólo quedan las columnas explícitas, del mismo ancho que entonces.
        Array.prototype.forEach.call(mosaic.children, function (item) {
            item.style.gridRow = "";
            item.style.gridColumn = "";
            item.style.marginTop = "";
        });
        var style = getComputedStyle(mosaic);
        // Sin rejilla (al imprimir, o si no ha cargado `mosaic.min.css`) quedan las columnas CSS, y
        // los desplazamientos sólo las descolocarían.
        if (!style.display.endsWith("grid")) {
            return;
        }
        var columns = style.gridTemplateColumns.split(" ").filter(Boolean).length || 1;
        // Espacio que queda bajo cada bloque (`--mosaic-row-gap`, a través de `row-gap`).
        var gap = parseFloat(style.rowGap) || 0;
        // Registrada con `@property` (ver `mosaic.scss`), llega ya en píxeles aunque se escriba en
        // `em`; si el navegador no la registra, llega tal cual y vale como píxeles.
        var tolerance = parseFloat(style.getPropertyValue("--mosaic-tolerance")) || 0;
        // Los bloques sin caja (`display: none`) no ocupan sitio.
        var items = Array.prototype.filter.call(mosaic.children, function (item) {
            return item.getClientRects().length > 0;
        });
        // Columnas que ocupa cada bloque, entre 1 y las que hay, y la columna a la que está fijado
        // (0 si no lo está). Las columnas se fijan antes de medir: un bloque más ancho mide
        // distinto.
        var spans = [];
        var pins = [];
        items.forEach(function (item) {
            var own = getComputedStyle(item);
            var span = own.getPropertyValue("--mosaic-span").trim() === "full"
                ? columns
                : Math.min(Math.max(integer(own, "--mosaic-span", 1), 1), columns);
            if (span > 1) {
                item.style.gridColumn = "span " + span;
            }
            spans.push(span);
            pins.push(integer(own, "--mosaic-column", 0));
        });
        var sizes = items.map(function (item) {
            return item.getBoundingClientRect().height;
        });
        var heights = new Array(columns).fill(0);

        // Altura a la que quedaría un bloque de `span` columnas que empieza en `column`.
        function bottom(column, span) {
            return Math.max.apply(null, heights.slice(column, column + span));
        }

        function place(i, column) {
            var top = bottom(column, spans[i]);
            var item = items[i];
            item.style.gridRow = "1";
            item.style.gridColumn = column + 1 + " / span " + spans[i];
            item.style.marginTop = top > 0 ? top + "px" : "";
            heights.fill(top + sizes[i] + gap, column, column + spans[i]);
        }

        // Los fijados a una columna (desde el principio si es positiva, desde el final si es
        // negativa), sin salirse del mosaico.
        var free = [];
        pins.forEach(function (pinned, i) {
            if (pinned === 0) {
                free.push(i);
                return;
            }
            var last = columns - spans[i];
            var column = pinned > 0 ? pinned - 1 : last + pinned + 1;
            place(i, Math.min(Math.max(column, 0), last));
        });

        var cursor = 0;
        free.forEach(function (i) {
            var span = spans[i];
            var lowest = Infinity;
            for (var c = 0; c + span <= columns; c++) {
                lowest = Math.min(lowest, bottom(c, span));
            }
            var first = -1;
            var next = -1;
            for (c = 0; c + span <= columns; c++) {
                if (bottom(c, span) <= lowest + tolerance) {
                    if (first < 0) {
                        first = c;
                    }
                    if (next < 0 && c >= cursor) {
                        next = c;
                    }
                }
            }
            var column = next >= 0 ? next : first;
            place(i, column);
            cursor = column + span;
        });

        mosaic.dispatchEvent(new CustomEvent("mosaic:layout", { bubbles: true }));
    }

    function init(mosaic) {
        if (mosaics.has(mosaic)) {
            return;
        }
        mosaics.add(mosaic);
        mosaic.classList.add("mosaic-js");
        // Al observarlo, cada bloque avisa de su tamaño y el mosaico se reparte.
        Array.prototype.forEach.call(mosaic.children, function (item) {
            resizer.observe(item);
        });
    }

    // Aplica `action` al propio elemento, si es un mosaico, y a los mosaicos que contenga.
    function each(root, action) {
        if (root.matches(".mosaic")) {
            action(root);
        }
        root.querySelectorAll(".mosaic").forEach(action);
    }

    // Deja de observar los bloques de un mosaico que sale de la página.
    function release(mosaic) {
        if (mosaics.has(mosaic)) {
            mosaics.delete(mosaic);
            Array.prototype.forEach.call(mosaic.children, function (item) {
                resizer.unobserve(item);
            });
        }
    }

    each(document.documentElement, init);

    // Al cambiar el ancho de la ventana puede activarse un punto de corte que cambia las columnas o
    // el espacio de un bloque sin que cambie el tamaño de ninguno (en un mosaico de ancho fijo, o
    // el espacio entre bloques de una sola columna), y entonces no avisaría ninguno: se repasan
    // todos los mosaicos, como mucho una vez por fotograma.
    var resizing = false;
    window.addEventListener("resize", function () {
        if (resizing) {
            return;
        }
        resizing = true;
        requestAnimationFrame(function () {
            resizing = false;
            document.querySelectorAll(".mosaic-js").forEach(function (mosaic) {
                if (mosaics.has(mosaic)) {
                    layout(mosaic);
                }
            });
        });
    });

    new MutationObserver(function (mutations) {
        // Mosaicos que han perdido bloques: se reparten una sola vez, aunque se quiten varios.
        var shrunk = new Set();
        mutations.forEach(function (mutation) {
            var mosaic = mutation.target;
            mutation.addedNodes.forEach(function (node) {
                if (node.nodeType !== Node.ELEMENT_NODE) {
                    return;
                }
                // Un bloque nuevo avisa de su tamaño al observarlo, y el mosaico se reparte.
                if (mosaics.has(mosaic)) {
                    resizer.observe(node);
                }
                each(node, init);
            });
            mutation.removedNodes.forEach(function (node) {
                if (node.nodeType !== Node.ELEMENT_NODE) {
                    return;
                }
                if (mosaics.has(mosaic)) {
                    resizer.unobserve(node);
                    shrunk.add(mosaic);
                }
                if (!node.isConnected) {
                    each(node, release);
                }
            });
        });
        shrunk.forEach(function (mosaic) {
            if (mosaic.isConnected) {
                layout(mosaic);
            }
        });
    }).observe(document.body, { childList: true, subtree: true });
})();
