
function toggleDarkMode() {
    const isDarkMode = document.body.classList.contains('dark');

    if (isDarkMode) {
        localStorage.setItem('dark-mode', 'false');
        document.body.classList.remove('dark');
        document.body.classList.add('light');
    }
    else {
        localStorage.setItem('dark-mode', 'true');
        document.body.classList.remove('light');
        document.body.classList.add('dark');
    }
}

function showSettings() {
    const settingsElement = document.createElement('div');
    settingsElement.id = 'backdrop';
    settingsElement.innerHTML = `
        <div id="webapp-settings-modal">
            <div class="flex-row justify-between">
                <h3 class="text-2x">Settings</h3>
                    
                <button class="button-rounded bg-burgundy text-2x button-events" style="width: 2.5rem; height: 2.5rem;" onclick="closeSettings()">
                    X
                </button>
            </div>

            <div class="flex-row">

                <button onclick="toggleDarkMode()">
                    Toggle Dark Mode
                </button>
            </div>
        </div>
    `;
    document.body.appendChild(settingsElement);
}

function closeSettings() {
    document.getElementById('backdrop').remove();
}

document.addEventListener('click', (event) => {
    if (event.target.id === 'backdrop') {
        document.body.removeChild(event.target);
    }
});