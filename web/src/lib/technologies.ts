import type { Technologies } from "./types";
import Rust from "../assets/technologies/rust.svg";
import CSharp from "../assets/technologies/csharp.svg";
import PHP from "../assets/technologies/php.svg";
import TypeScript from "../assets/technologies/typescript.svg";
import Svelte from "../assets/technologies/svelte.svg";
import Tailwindcss from "../assets/technologies/tailwindcss.svg";
import JavaScript from "../assets/technologies/javascript.svg";

const backend: Technologies[] = [
  {
    name: "Rust",
    src: Rust,
    url: "https://www.rust-lang.org/",
  },
  {
    name: "C#",
    src: CSharp,
    url: "https://docs.microsoft.com/en-us/dotnet/csharp/",
  },
  {
    name: "PHP",
    src: PHP,
    url: "https://www.php.net/",
  },
  {
    name: "TypeScript",
    src: TypeScript,
    url: "https://www.typescriptlang.org/",
  },
];

const frontend: Technologies[] = [
  {
    name: "Svelte",
    src: Svelte,
    url: "https://svelte.dev/",
  },
  {
    name: "Tailwindcss",
    src: Tailwindcss,
    url: "https://tailwindcss.com/",
  },
  {
    name: "TypeScript",
    src: TypeScript,
    url: "https://www.typescriptlang.org/",
  },
  {
    name: "JavaScript",
    src: JavaScript,
    url: "https://www.javascript.com/",
  },
];

export let techs: Technologies[] = [...backend, ...frontend];
