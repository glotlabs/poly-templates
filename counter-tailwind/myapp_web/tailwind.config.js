/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../myapp_core/**/*_page.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
