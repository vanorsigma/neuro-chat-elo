/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      gridTemplateColumns: {
        '4': '1fr 4fr 1fr 1fr'
      },
      colors: {
        'neuro': '#ad5411',
        'chat': '#e2936f',
        'on-bg': '#ffffff',
      }
    },
  },
  plugins: [],
}
