/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.hbs"],
  theme: {
    extend: {
      colors: {
        // Custom color palette extensions
        'rocket-blue': {
          50: '#e6f0ff',
          100: '#bdd6ff',
          200: '#94bbff',
          300: '#6ba0ff',
          400: '#4285fe',
          500: '#1a6bfa',
          600: '#0054e6',
          700: '#0042b3',
          800: '#003080',
          900: '#001e4d',
        },
      },
      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial', 'sans-serif'],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      }
    },
  },
  plugins: [],
}
