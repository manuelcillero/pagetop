(function () {
    'use strict';

    // Bootstrap's `.modal` is a plain `position: fixed` element: it never leaves the CSS stacking
    // context of whatever ancestor it was rendered inside. If any ancestor sets its own `z-index`
    // (e.g. pagetop's `Intro` component, for its decorative layering), the modal's `z-index` only
    // ranks it among the elements of *that* ancestor's context, so `.modal-backdrop` -always a
    // direct child of `<body>`, added by Bootstrap itself- can end up rendering above it and
    // swallowing every click, even though `.modal`'s own `z-index` is numerically higher. Basic's
    // native `<dialog>` escapes this for free via the browser's "top layer"; Bootsier's `.modal`
    // does not, so it is moved to `<body>` here, exactly like Bootstrap already does with its own
    // backdrop.
    document.querySelectorAll('.modal.dialog').forEach(function (modal) {
        if (modal.parentElement !== document.body) {
            document.body.appendChild(modal);
        }
    });
}());
