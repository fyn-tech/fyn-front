/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs", 
    "./index.html",
  ],
  darkMode: 'class', // or 'media' if you prefer system preference
  theme: {
    extend: {
      fontFamily: {
        'primary': ['Inter', 'system-ui', 'sans-serif'],     // Modern sans-serif
        'technical': ['JetBrains Mono', 'monospace'],        // For data/code
      },
      colors: {
        // Your Table Bay Blue primary
        primary: {
          50: '#BAE6FD',   // primary-light
          300: '#7BC8E8',
          500: '#0891B2',  // primary (DEFAULT)
          700: '#0E7490',
          950: '#164E63',  // primary-dark
        },
        // Your Emerald accent  
        accent: {
          50: '#A7F3D0',   // accent-light
          300: '#6EE7B7',
          500: '#10B981',  // accent (DEFAULT)
          700: '#047857',
          950: '#065F46',  // accent-dark
        },
        // Your Table Mountain neutrals
        surface: {
          50: '#FFFFFF',   // surface-0 (light page bg)
          100: '#F8FAFC',  // surface-1 (light cards)
          200: '#E2E8F0',  // surface-2 (light borders)
          300: '#94A3B8',  // surface-3 (inactive)
          700: '#475569',  // dark-surface-3 (dark inactive)
          800: '#334155',  // dark-surface-2 (dark borders)
          900: '#1E293B',  // dark-surface-1 (dark cards)
          950: '#0F172A',  // dark-surface-0 (dark page bg)
        },
        content: {
          primary: '#2D3748',     // text-primary
          secondary: '#475569',   // text-secondary  
          tertiary: '#94A3B8',    // text-tertiary
          'primary-dark': '#E2E8F0',   // dark-text-primary
          'secondary-dark': '#94A3B8', // dark-text-secondary
          'tertiary-dark': '#64748B',  // dark-text-tertiary
        },
        semantic: {
          // Your semantic colors
          success: '#059669',
          warning: '#D97706', 
          error: '#DC2626',
        }
      }
    },
  },
  plugins: [],
  safelist: [
    'text-content-primary-dark',
    'dark:text-content-primary-dark'  // Add the dark variant too
  ],
}