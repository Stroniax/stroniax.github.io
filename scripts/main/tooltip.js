function registerTooltipCode() {
    document.querySelectorAll('[tooltip]').forEach(node => {
        node.addEventListener('mouseenter', (event) => {
            const tooltip = document.createElement('div');
            tooltip.classList.add('tooltip');
            tooltip.innerText = node.getAttribute('tooltip');
            tooltip.offsetTop = event.clientY;
            tooltip.offsetLeft = event.clientX;
            node.appendChild(tooltip);
        });
        node.addEventListener('mouseleave', () => {
            node.querySelector('.tooltip').remove();
        });
    });
}
window.addEventListener('load', registerTooltipCode);