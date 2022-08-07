// @ts-check
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,jsx,ts,tsx,vue}"],
  theme: {
    extend: {
      gridTemplateColumns: {
        "30/70": "30% 70%",
      },
    },
  },
  plugins: [require("@tailwindcss/forms"), require("daisyui")],
};
