/** @type {import("tailwindcss").Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      backgroundColor: {
        "gray": "rgba(0, 0, 0, 0.02)",
        "dark": "rgba(0, 0, 0, 0.93)",
      }
    },
  },
  plugins: [],
}
