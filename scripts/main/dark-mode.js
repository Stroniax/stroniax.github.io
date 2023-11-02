function isDarkMode() {
    return document.body?.classList.contains('dark');
}

function getDarkModePreference() {
    const systemPreference = window.matchMedia?.('(prefers-color-scheme: dark)').matches;
    const localPreference = localStorage.getItem('dark-mode');
    const calculatedPreference = localPreference === null
        ? systemPreference
        : localPreference === 'true';

    return {
        systemPreference,
        localPreference,
        calculatedPreference,
    };
}

function setDarkMode(value, storePreference = true) {
    if (document.body === null) {
        throw new Error('document.body is null; setDarkMode must be called after the document has loaded.');
    }

    const removeClass = value ? 'light' : 'dark';
    const addClass = value ? 'dark' : 'light';
    document.body.classList.remove(removeClass);
    document.body.classList.add(addClass);

    if (storePreference) {
        localStorage.setItem('dark-mode', value ? 'true' : 'false');
    }
}

function toggleDarkMode() {
    setDarkMode(!isDarkMode());
}

function initializeDarkMode() {
    const preference = getDarkModePreference();

    setDarkMode(preference.calculatedPreference, preference.systemPreference);
}

function watchDarkMode() {
    initializeDarkMode();

    function onSystemDarkModePreferenceChange(event) {
        setDarkMode(event.matches, false);
    }

    window
        .matchMedia?.('(prefers-color-scheme: dark)')
        .addEventListener('change', onSystemDarkModePreferenceChange);
}

window.addEventListener('load', watchDarkMode);