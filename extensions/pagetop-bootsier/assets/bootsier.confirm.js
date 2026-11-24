(function () {
    'use strict';

    // htmx dispatches `htmx:confirm` in place of `window.confirm()` whenever the element that
    // would issue a request carries `hx-confirm`. Intercepting it here upgrades every
    // `hx-confirm` in the project to a Bootstrap modal, without touching the element that
    // requested it or the request itself: cancelling is a no-op, confirming resumes the exact
    // same request via `event.detail.issueRequest(true)`.
    if (typeof bootstrap === 'undefined' || !bootstrap.Modal) { return; }

    var modalEl = null;
    var questionEl = null;
    var pendingEvent = null;

    function ensureModal() {
        if (modalEl) { return; }

        var okLabel = document.body.dataset.confirmOk || 'OK';
        var cancelLabel = document.body.dataset.confirmCancel || 'Cancel';

        modalEl = document.createElement('div');
        modalEl.className = 'modal fade';
        modalEl.tabIndex = -1;
        modalEl.setAttribute('aria-hidden', 'true');
        modalEl.innerHTML =
            '<div class="modal-dialog">' +
                '<div class="modal-content">' +
                    '<div class="modal-body"></div>' +
                    '<div class="modal-footer">' +
                        '<button type="button" class="btn btn-secondary" data-bs-dismiss="modal"></button>' +
                        '<button type="button" class="btn btn-danger" data-confirm-accept></button>' +
                    '</div>' +
                '</div>' +
            '</div>';
        document.body.appendChild(modalEl);

        questionEl = modalEl.querySelector('.modal-body');
        modalEl.querySelector('[data-bs-dismiss]').textContent = cancelLabel;
        var acceptBtn = modalEl.querySelector('[data-confirm-accept]');
        acceptBtn.textContent = okLabel;

        acceptBtn.addEventListener('click', function () {
            var pending = pendingEvent;
            pendingEvent = null;
            bootstrap.Modal.getInstance(modalEl).hide();
            if (pending) { pending.detail.issueRequest(true); }
        });

        modalEl.addEventListener('hidden.bs.modal', function () {
            pendingEvent = null;
        });
    }

    document.addEventListener('htmx:confirm', function (event) {
        // Already handled by someone else, such as the shared dialog of `pagetop-htmx`
        // (`hx_confirm`), which listens in the capture phase.
        if (!event.detail.question || event.defaultPrevented) { return; }
        event.preventDefault();
        ensureModal();
        questionEl.textContent = event.detail.question;
        pendingEvent = event;
        bootstrap.Modal.getOrCreateInstance(modalEl).show();
    });
}());
