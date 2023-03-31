import "./styles.css";
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app"),
});

export default app;

export function sleep(ms) {return new Promise(resolve => setTimeout(resolve, ms));}