// Main JavaScript for the Rocket web application

document.addEventListener('DOMContentLoaded', () => {
    // Add animation for page elements
    const mainContent = document.querySelector('main > div');
    if (mainContent) {
        mainContent.style.opacity = '0';
        setTimeout(() => {
            mainContent.style.opacity = '1';
            mainContent.style.transition = 'opacity 0.5s ease-out';
        }, 100);
    }

    // Initialize any Alpine.js components if needed
    if (window.Alpine) {
        // Alpine is ready
        console.log('Alpine.js initialized');
    }
});
