(function () {
    'use strict';

    // Fullscreen: keeps maximize/minimize icons in sync with the actual fullscreen state.
    document.addEventListener('fullscreenchange', function () {
        var isFs = !!document.fullscreenElement;
        document.querySelectorAll('[data-lte-icon="maximize"]').forEach(function (el) {
            el.classList.toggle('d-none', isFs);
        });
        document.querySelectorAll('[data-lte-icon="minimize"]').forEach(function (el) {
            el.classList.toggle('d-none', !isFs);
        });
    });

    // Color mode selector (light / dark / auto).
    var STORAGE_KEY = 'lte-theme';
    var getStored = function () { return localStorage.getItem(STORAGE_KEY); };
    var setStored = function (theme) { localStorage.setItem(STORAGE_KEY, theme); };
    var prefersDark = function () {
        return window.matchMedia('(prefers-color-scheme: dark)').matches;
    };

    var setTheme = function (theme) {
        var resolved = (theme === 'auto') ? (prefersDark() ? 'dark' : 'light') : theme;
        document.documentElement.setAttribute('data-bs-theme', resolved);
    };

    var showActiveTheme = function (theme) {
        document.querySelectorAll('[data-bs-theme-value]').forEach(function (el) {
            el.classList.remove('active');
            el.setAttribute('aria-pressed', 'false');
            var check = el.querySelector('.bi-check-lg');
            if (check) { check.classList.add('d-none'); }
        });
        var active = document.querySelector('[data-bs-theme-value="' + theme + '"]');
        if (active) {
            active.classList.add('active');
            active.setAttribute('aria-pressed', 'true');
            var check = active.querySelector('.bi-check-lg');
            if (check) { check.classList.remove('d-none'); }
        }
        document.querySelectorAll('[data-lte-theme-icon]').forEach(function (icon) {
            icon.classList.toggle('d-none', icon.dataset.lteThemeIcon !== theme);
        });
    };

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function () {
        var stored = getStored();
        if (!stored || stored === 'auto') { setTheme('auto'); }
    });

    document.addEventListener('DOMContentLoaded', function () {
        var theme = getStored() || 'auto';
        setTheme(theme);
        showActiveTheme(theme);
        document.querySelectorAll('[data-bs-theme-value]').forEach(function (toggle) {
            toggle.addEventListener('click', function () {
                var t = toggle.getAttribute('data-bs-theme-value');
                setStored(t);
                setTheme(t);
                showActiveTheme(t);
            });
        });
    });
}());
