/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../myapp_lib/**/*_page.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
