function menuShowChildren(nav, children) {
	let submenu = children[0].querySelector('.menu-subs');
	submenu.classList.add('active');
	submenu.style.animation = 'slideLeft 0.5s ease forwards';

	let title = children[0].querySelector('i').parentNode.childNodes[0].textContent;
	nav.querySelector('.menu-title').innerHTML = title;
	nav.querySelector('.menu-header').classList.add('active');
}

function menuHideChildren(nav, children) {
	let submenu = children[0].querySelector('.menu-subs');
	submenu.style.animation = 'slideRight 0.5s ease forwards';
	setTimeout(() => {
		submenu.classList.remove('active');
	}, 300);

	children.shift();
	if (children.length > 0) {
		let title = children[0].querySelector('i').parentNode.childNodes[0].textContent;
		nav.querySelector('.menu-title').innerHTML = title;
	} else {
		nav.querySelector('.menu-header').classList.remove('active');
		nav.querySelector('.menu-title').innerHTML = '';
	}
}

function menuToggle(nav, overlay) {
	nav.classList.toggle('active');
	overlay.classList.toggle('active');
}

document.querySelectorAll('.menu-container').forEach(menu => {

	let menuChildren = [];
	const menuNav = menu.querySelector('.menu');
	const menuOverlay = menu.querySelector('.menu-overlay');

	menu.querySelector('.menu-section').addEventListener('click', (e) => {
		if (!menuNav.classList.contains('active')) {
			return;
		}
		let target = e.target.closest('.menu-children');
		if (target && target != menuChildren[0]) {
			menuChildren.unshift(target);
			menuShowChildren(menuNav, menuChildren);
		}
	});
	menu.querySelector('.menu-arrow').addEventListener('click', () => {
		menuHideChildren(menuNav, menuChildren);
	});
	menu.querySelector('.menu-close').addEventListener('click', () => {
		menuToggle(menuNav, menuOverlay);
		setTimeout(() => {
			menuNav.querySelector('.menu-header').classList.remove('active');
			menuNav.querySelector('.menu-title').innerHTML = '';
			menu.querySelectorAll('.menu-subs').forEach(close => {
				close.classList.remove('active');
			});
			menuChildren = [];
		}, 300);
	});
	menu.querySelector('.menu-trigger').addEventListener('click', () => {
		menuToggle(menuNav, menuOverlay);
	});

	menuOverlay.addEventListener('click', () => {
		menuToggle(menuNav, menuOverlay);
	});

	window.onresize = function () {
		if (this.innerWidth >= 992) {
			if (menuNav.classList.contains('active')) {
				menuToggle(menuNav, menuOverlay);
			}
		}
	};
});
