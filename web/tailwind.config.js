/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      gridTemplateColumns: {
        '4': '1fr 4fr 1fr 1fr'
      },
      colors: {
        'neuro': '#fea8ae',
        'chat': '#f7dedf'
      }
    },
  },
  plugins: [],
}
