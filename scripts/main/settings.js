function openSettings() {
    const settingsElement = document.createElement('div');
    settingsElement.id = 'backdrop';
    settingsElement.innerHTML = `
        <div id="webapp-settings-modal">
            <div class="flex-row flex-align-center justify-between">
                <h3 class="text-2x">Settings</h3>
                    
                <button class="button-rounded bg-burgundy text-2x button-events" style="width: 2.5rem; height: 2.5rem;" onclick="closeSettings()">
                    X
                </button>
            </div>

            <div class="flex-col gap-2">

                <div class="flex-row justify-between rounded-sm bg-default">
                    <button class="flex-1 bg-dark rounded-sm-left" onclick="setDarkMode(true, true)">
                        Dark
                    </button>

                    <button class="flex-1 bg-light border-left border-right" onclick="setDarkMode(false, true)">
                        Light
                    </button>

                    <button class="flex-1 bg-system rounded-sm-right" onclick="resetDarkMode()">
                        Use System Preference (${getDarkModePreference().systemPreference ? 'Dark' : 'Light'})
                    </button>
                </div>

                <button class="button-action bg-warn" onclick="resetSettings()">
                    Reset Settings
                </button>

                <div class="flex-row">
                    <button class="button-action" style="background-color: #3a3a3a;" onclick="closeSettings()">
                        Close
                    </button>
                </div>
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