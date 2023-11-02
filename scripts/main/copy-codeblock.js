
function copyCodeBlock() {
    const clip = navigator.clipboard;
    const items = [
        new ClipboardItem({
            'text/plain': new Blob([this.innerText], { type: 'text/plain' }),
            'text/html': new Blob([this.innerHTML], { type: 'text/html' })
        }),
    ];

    clip.write(items)
        .then(() => {
            const copiedElement = document.createElement('p');
            copiedElement.classList.add('alert-text');
            copiedElement.innerText = 'Copied!';
            alertContainer.appendChild(copiedElement);
            setTimeout(() => {
                copiedElement.classList.add('animate-fade-out');
                setTimeout(() => {
                    copiedElement.remove();
                }, 1000);
            }, 2000);
        })
        .catch((e) => console.error('failed to set clipboard', e));

}

document.addEventListener('DOMContentLoaded', () => {
    const codeBlocks = [
        ...document.getElementsByClassName('code-block'),
        ...document.getElementsByClassName('code-inline'),
    ];
    for (const codeBlock of codeBlocks) {
        codeBlock.classList.add('cursor-pointer');
        codeBlock.addEventListener('click', copyCodeBlock);
    }
});

let alertContainer;

function buildAlertContainer() {
    alertContainer = document.createElement('div');
    alertContainer.id = 'alert-container';
    document.body.appendChild(alertContainer);
}

window.addEventListener('load', buildAlertContainer);

document.addEventListener('click', (event) => {
    if (event.target.classList.contains('alert-text')) {
        event.target.remove();
    }
});