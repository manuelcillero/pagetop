const navChapters = document.querySelectorAll('#mdbook a.nav-chapters');

const calculatePosition = (navChapter) => {
    const windowHeight = window.innerHeight;
    const navRect = navChapter.getBoundingClientRect();
    const navIcon = navChapter.querySelector('i.fa');
    const iRect = navIcon.getBoundingClientRect();

    // Calculate the vertical central position
    let iMiddle = navRect.height / 2 - iRect.height / 2;

    if (iMiddle + navRect.top > windowHeight - iRect.height) {
        // Positions below until upper edge
        iMiddle = Math.max(windowHeight - navRect.top - iRect.height, 0);
    } else if (iMiddle + navRect.top < 0) {
        // Positions above until lower edge
        iMiddle = Math.min(Math.abs(navRect.top), navRect.height - iRect.height);
    }

    navIcon.style.transform = 'translateY(' + iMiddle + 'px)';
    navIcon.style.top = iMiddle + 'px';
    navIcon.style.visibility = 'visible';
};

// Initial position
navChapters.forEach((navChapter) => {
    calculatePosition(navChapter);
});

// Add a scroll event listener
window.addEventListener('scroll', () => {
    navChapters.forEach((navChapter) => {
        calculatePosition(navChapter);
    });
});

// Add a resize event listener
window.addEventListener('resize', () => {
    navChapters.forEach((navChapter) => {
        calculatePosition(navChapter);
    });
});
