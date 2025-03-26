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

    // Function to toggle dark mode
    function toggleDarkMode() {
        document.body.classList.toggle('dark-mode');
        document.querySelector('header').classList.toggle('dark-mode');
        document.querySelector('footer').classList.toggle('dark-mode');
        document.querySelector('nav').classList.toggle('dark-mode');
        document.querySelector('main').classList.toggle('dark-mode');
        document.querySelectorAll('section').forEach(section => section.classList.toggle('dark-mode'));

        // Save preference in local storage
        if (document.body.classList.contains('dark-mode')) {
            localStorage.setItem('darkMode', 'enabled');
        } else {
            localStorage.setItem('darkMode', 'disabled');
        }
    }

    // Event listener for dark mode toggle button
    const darkModeToggle = document.querySelector('.dark-mode-toggle');
    if (darkModeToggle) {
        darkModeToggle.addEventListener('click', toggleDarkMode);
    }

    // Check and apply dark mode preference on page load
    if (localStorage.getItem('darkMode') === 'enabled') {
        toggleDarkMode();
    }
});
