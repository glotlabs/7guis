/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../sevenguis_core/**/*_page.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
