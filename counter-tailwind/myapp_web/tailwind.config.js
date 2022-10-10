/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../myapp_core/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
