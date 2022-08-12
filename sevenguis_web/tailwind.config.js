/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../sevenguis_lib/**/*_page.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
