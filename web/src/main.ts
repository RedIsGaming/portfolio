import "./tailwind.css";
import "./custom.css";
import App from "./App.svelte";

const app: App = new App({
  target: document.getElementById("app")!,
});

export default app;
