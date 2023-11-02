function setDarkMode(value) {
    if (value) {
        localStorage.setItem('dark-mode', 'true');
        document.body.classList.remove('light');
        document.body.classList.add('dark');
    }
    else {
        localStorage.setItem('dark-mode', 'false');
        document.body.classList.remove('dark');
        document.body.classList.add('light');
    }
}

function toggleDarkMode() {
    const isDarkMode = document.body?.classList.contains('dark');
    setDarkMode(!isDarkMode);
}

function onDarkModeChange(event) {
    setDarkMode(event.matches);
}

function initializeDarkMode() {
    const isSystemDarkMode = window.matchMedia?.('(prefers-color-scheme: dark)').matches;
    const darkModePreference = localStorage.getItem('dark-mode');

    if (darkModePreference === null) {
        setDarkMode(isSystemDarkMode);
    }
    else {
        setDarkMode(darkModePreference === 'true');
    }
}

function watchDarkMode() {
    initializeDarkMode();

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', onDarkModeChange);
}

window.addEventListener('load', watchDarkMode);