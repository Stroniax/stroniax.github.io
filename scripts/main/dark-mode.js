function setDarkMode(value) {
    if (value) {
        document.body.classList.remove('light');
        document.body.classList.add('dark');
    }
    else {
        document.body.classList.remove('dark');
        document.body.classList.add('light');
    }
}

function onDarkModeChange(event) {
    setDarkMode(event.matches);
}

function watchDarkMode() {
    const isSystemDarkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    const darkModePreference = localStorage.getItem('dark-mode');
    if (darkModePreference === undefined) {
        setDarkMode(isSystemDarkMode);
    }
    else {
        setDarkMode(darkModePreference === 'true');
    }

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', onDarkModeChange);
}

window.addEventListener('load', watchDarkMode);