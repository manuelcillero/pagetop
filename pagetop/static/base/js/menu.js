function pt_menu__showChildren(nav, children) {
	let submenu = children[0].querySelector('.pt-menu__subs');
	submenu.classList.add('active');
	submenu.style.animation = 'slideLeft 0.5s ease forwards';

	let title = children[0].querySelector('i').parentNode.childNodes[0].textContent;
	nav.querySelector('.pt-menu__title').innerHTML = title;
	nav.querySelector('.pt-menu__header').classList.add('active');
}

function pt_menu__hideChildren(nav, children) {
	let submenu = children[0].querySelector('.pt-menu__subs');
	submenu.style.animation = 'slideRight 0.5s ease forwards';
	setTimeout(() => {
		submenu.classList.remove('active');
		submenu.style.removeProperty("animation");
	}, 300);

	children.shift();
	if (children.length > 0) {
		let title = children[0].querySelector('i').parentNode.childNodes[0].textContent;
		nav.querySelector('.pt-menu__title').innerHTML = title;
	} else {
		nav.querySelector('.pt-menu__header').classList.remove('active');
		nav.querySelector('.pt-menu__title').innerHTML = '';
	}
}

function pt_menu__toggle(nav, overlay) {
	nav.classList.toggle('active');
	overlay.classList.toggle('active');
}

function pt_menu__reset(menu, nav) {
	nav.querySelector('.pt-menu__header').classList.remove('active');
	nav.querySelector('.pt-menu__title').innerHTML = '';
	menu.querySelectorAll('.pt-menu__subs').forEach(submenu => {
		submenu.classList.remove('active');
		submenu.style.removeProperty("animation");
	});
}

document.querySelectorAll('.pt-menu__container').forEach(menu => {

	let menuChildren = [];
	const menuNav = menu.querySelector('.pt-menu__nav');
	const menuOverlay = menu.querySelector('.pt-menu__overlay');

	menu.querySelector('.pt-menu__section').addEventListener('click', (e) => {
		if (!menuNav.classList.contains('active')) {
			return;
		}
		let target = e.target.closest('.pt-menu__children');
		if (target && target != menuChildren[0]) {
			menuChildren.unshift(target);
			pt_menu__showChildren(menuNav, menuChildren);
		}
	});

	menu.querySelector('.pt-menu__arrow').addEventListener('click', () => {
		pt_menu__hideChildren(menuNav, menuChildren);
	});

	menu.querySelector('.pt-menu__close').addEventListener('click', () => {
		pt_menu__toggle(menuNav, menuOverlay);
		setTimeout(() => {
			pt_menu__reset(menu, menuNav);
			menuChildren = [];
		}, 300);
	});

	menu.querySelector('.pt-menu__trigger').addEventListener('click', () => {
		pt_menu__toggle(menuNav, menuOverlay);
	});

	menuOverlay.addEventListener('click', () => {
		pt_menu__toggle(menuNav, menuOverlay);
	});

	window.onresize = function () {
		if (this.innerWidth >= 992) {
			if (menuNav.classList.contains('active')) {
				pt_menu__toggle(menuNav, menuOverlay);
				setTimeout(() => {
					pt_menu__reset(menu, menuNav);
					menuChildren = [];
				}, 300);
			}
		}
	};
});
