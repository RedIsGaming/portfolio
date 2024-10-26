/** @type {import("tailwindcss").Config} */
export default {
  content: ["./src/**/*.{html,js,rs}", "index.html"],
  theme: {
    extend: {
      fontFamily: {
        mono: ["Monaco"],
      },
    },
  },
  plugins: [],
}
