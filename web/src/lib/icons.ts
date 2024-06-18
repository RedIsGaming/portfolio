import { CodeBracket, Home } from "svelte-hero-icons";
import type { Icon } from "./types";
import { github, discord } from "./socials";
import Github from "../assets/platforms/github.svg";
import Discord from "../assets/platforms/discord.svg";

function icons(icon: Icon): Icon {
  return { ...icon, href: "/", size: 48 };
}

const logo: Icon[] = [{ name: "logo", src: CodeBracket }];
const home: Icon[] = [{ name: "home", src: Home }];
const social: Icon[] = [
  {
    name: github,
    href: `https://${github.toLowerCase()}.com/RedIsGaming/portfolio`,
    src: Github,
  },
  {
    name: discord,
    href: `https://${discord.toLowerCase()}app.com/users/724341024415285319`,
    src: Discord,
  },
];

export const logos: Icon[] = logo.map(icons);
export const homes: Icon[] = home.map(icons);
export const socials: Icon[] = social;
