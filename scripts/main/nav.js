function loadNavigation() {
    const navElement = document.getElementById('nav');

    const html = `
        <div class="flex-col">
            <a href="" class="nav-item nav-never-highlight">Home</a>
            <a href="about" class="nav-item">About</a>
            <a href="contact" class="nav-item">Contact</a>
        </div>

        <div class="flex-col">
            <a href="series" class="nav-item">Series</a>
            <a href="series/war" class="nav-item">Learning with a Game of War</a>
        </div>
    `;
    navElement.innerHTML = html;

    const navLinks = navElement.getElementsByTagName('a');
    let previousItem = null;
    for (const navLink of navLinks) {
        if (window.location.href.startsWith(navLink.href)) {
            previousItem?.classList.remove('nav-active');
            previousItem = navLink;
            navLink.classList.add('nav-active');
        }
    }
}

window.addEventListener('load', loadNavigation);