/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs", 
    "./index.html",
  ],
  darkMode: 'media',
  theme: {
    extend: {
      colors: {
        primary: {
          light: '#E0F2FE',
          DEFAULT: '#0891B2', 
          dark: '#164E63',
        },
        accent: {
          light: '#CFFAFE',
          DEFAULT: '#10B981',
          dark: '#065F46',
        },
        sematic: {
          success: '#059669',
          warning: '#D97706',
          error: '#DC2626',
        }
      },
    },
  },
  plugins: [],
}