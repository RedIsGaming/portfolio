import { CodeBracket, Home } from "svelte-hero-icons";
import type { Icon } from "./types";
import { github, discord } from "./socials";

function icons(icon: Icon): Icon {
  return { ...icon, href: "/", size: 48 };
}

const logo: Icon[] = [{ name: "logo", src: CodeBracket }];
const home: Icon[] = [{ name: "home", src: Home }];
const social: Icon[] = [
  {
    name: github,
    href: `https://${github.toLowerCase()}.com/RedIsGaming/`,
    src: `/src/assets/platforms/${github.toLowerCase()}.svg`,
  },
  {
    name: discord,
    href: `https://${discord.toLowerCase()}app.com/users/724341024415285319`,
    src: `/src/assets/platforms/${discord.toLowerCase()}.svg`,
  },
];

export let logos: Icon[] = logo.map(icons);
export let homes: Icon[] = home.map(icons);
export let socials: Icon[] = social;
