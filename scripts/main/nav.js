function loadNavigation() {
    const navElement = document.getElementById('nav');


    fetch('./nav.html')
        .then(response => response.text())
        .then(response => navElement.innerHTML = response)
        .catch(() => console.error('Failed to load navigation'));
}

window.addEventListener('load', loadNavigation);