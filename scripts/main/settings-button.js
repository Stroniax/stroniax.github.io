function openSettings() {
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

            <div class="flex-col">

                <button onclick="resetSettings()">
                    Reset Settings
                </button>

                <button onclick="toggleDarkMode()">
                    Toggle Dark Mode
                </button>
            </div>
        </div>
    `;
    document.body.appendChild(settingsElement);
}

function resetSettings() {
    localStorage.clear();
    initializeDarkMode();
}

function closeSettings() {
    document.getElementById('backdrop').remove();
}

document.addEventListener('click', (event) => {
    if (event.target.id === 'backdrop') {
        document.body.removeChild(event.target);
    }
});