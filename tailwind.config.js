/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      animation: {
        fade: 'fadeOut 0.5s ease-in-out',
      },
      keyframes: {
        fadeOut: {
          '0%': { opacity: "100%" },
          '100%': { opacity: "0%" },
        },
      },
    },
  },
  plugins: [],
}

