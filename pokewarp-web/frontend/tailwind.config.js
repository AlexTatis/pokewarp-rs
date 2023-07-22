/** @type {import('tailwindcss').Config} */
export default {
  content: [],
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["lofi",
      {
        mytheme: {

          "primary": "#334155",
          "secondary": "#1a1919",
          "accent": "#262626",
          "neutral": "#000000",
          "base-100": "#ffffff",
          "info": "#0072f5",
          "success": "#21ca51",
          "warning": "#ff6052",
          "error": "#de1b8d",
        },
      },]
  }
}

