/** @type {import('tailwindcss').Config} */
export const content = [
  './app/**/*.{js,ts,jsx,tsx,mdx}',
  './pages/**/*.{js,ts,jsx,tsx,mdx}',
  './components/**/*.{js,ts,jsx,tsx,mdx}',
  './src/**/*.{js,ts,jsx,tsx,mdx}',
];
export const theme = {
  extend: {
    colors: {
      'bg-primary': 'var(--bg-primary)',
      'bg-secondary': 'var(--bg-secondary)',
      'bg-tertiary': 'var(--bg-tertiary)',
      'accent-primary': 'var(--accent-primary)',
      'accent-secondary': 'var(--accent-secondary)',
      'accent-tertiary': 'var(--accent-tertiary)',
      'text-primary': 'var(--text-primary)',
      'text-secondary': 'var(--text-secondary)',
    },
  },
};
export const plugins = [];