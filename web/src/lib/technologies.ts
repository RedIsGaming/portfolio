import type { Technologies } from "./types";

const backend: Technologies[] = [
  {
    name: "Rust",
    src: "/src/assets/technologies/rust.svg",
    url: "https://www.rust-lang.org/",
  },
  {
    name: "C#",
    src: "/src/assets/technologies/csharp.svg",
    url: "https://docs.microsoft.com/en-us/dotnet/csharp/",
  },
  {
    name: "PHP",
    src: "/src/assets/technologies/php.svg",
    url: "https://www.php.net/",
  },
  {
    name: "TypeScript",
    src: "/src/assets/technologies/typescript.svg",
    url: "https://www.typescriptlang.org/",
  },
];

const frontend: Technologies[] = [
  {
    name: "Svelte",
    src: "/src/assets/technologies/svelte.svg",
    url: "https://svelte.dev/",
  },
  {
    name: "Tailwindcss",
    src: "/src/assets/technologies/tailwindcss.svg",
    url: "https://tailwindcss.com/",
  },
  {
    name: "TypeScript",
    src: "/src/assets/technologies/typescript.svg",
    url: "https://www.typescriptlang.org/",
  },
  {
    name: "JavaScript",
    src: "/src/assets/technologies/javascript.svg",
    url: "https://www.javascript.com/",
  },
];

export let techs: Technologies[] = [...backend, ...frontend];
