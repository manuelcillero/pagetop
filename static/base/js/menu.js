function menu__showChildren(nav, children) {
	let submenu = children[0].querySelector('.menu__subs');
	submenu.classList.add('active');
	submenu.style.animation = 'slideLeft 0.5s ease forwards';

	let title = children[0].querySelector('i').parentNode.childNodes[0].textContent;
	nav.querySelector('.menu__title').innerHTML = title;
	nav.querySelector('.menu__header').classList.add('active');
}

function menu__hideChildren(nav, children) {
	let submenu = children[0].querySelector('.menu__subs');
	submenu.style.animation = 'slideRight 0.5s ease forwards';
	setTimeout(() => {
		submenu.classList.remove('active');
		submenu.style.removeProperty("animation");
	}, 300);

	children.shift();
	if (children.length > 0) {
		let title = children[0].querySelector('i').parentNode.childNodes[0].textContent;
		nav.querySelector('.menu__title').innerHTML = title;
	} else {
		nav.querySelector('.menu__header').classList.remove('active');
		nav.querySelector('.menu__title').innerHTML = '';
	}
}

function menu__toggle(nav, overlay) {
	nav.classList.toggle('active');
	overlay.classList.toggle('active');
}

function menu__reset(menu, nav) {
	nav.querySelector('.menu__header').classList.remove('active');
	nav.querySelector('.menu__title').innerHTML = '';
	menu.querySelectorAll('.menu__subs').forEach(submenu => {
		submenu.classList.remove('active');
		submenu.style.removeProperty("animation");
	});
}

document.querySelectorAll('.menu__container').forEach(menu => {

	let menuChildren = [];
	const menuNav = menu.querySelector('.menu__nav');
	const menuOverlay = menu.querySelector('.menu__overlay');

	menu.querySelector('.menu__section').addEventListener('click', (e) => {
		if (menuNav.classList.contains('active')) {
			let target = e.target.closest('.menu__children');
			if (target && target != menuChildren[0]) {
				menuChildren.unshift(target);
				menu__showChildren(menuNav, menuChildren);
			}
		}
	});

	menu.querySelector('.menu__arrow').addEventListener('click', () => {
		menu__hideChildren(menuNav, menuChildren);
	});

	menu.querySelector('.menu__close').addEventListener('click', () => {
		menu__toggle(menuNav, menuOverlay);
		setTimeout(() => {
			menu__reset(menu, menuNav);
			menuChildren = [];
		}, 300);
	});

	menu.querySelector('.menu__trigger').addEventListener('click', () => {
		menu__toggle(menuNav, menuOverlay);
	});

	menuOverlay.addEventListener('click', () => {
		menu__toggle(menuNav, menuOverlay);
	});

	window.onresize = function () {
		if (this.innerWidth >= 992) {
			if (menuNav.classList.contains('active')) {
				menu__toggle(menuNav, menuOverlay);
				setTimeout(() => {
					menu__reset(menu, menuNav);
					menuChildren = [];
				}, 300);
			}
		}
	};
});
