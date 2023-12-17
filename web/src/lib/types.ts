import { IconSource } from "svelte-hero-icons";

export type CustomIcon = {
  name?: string,
  href?: string,
  src: IconSource | string,
  size?: number,
};

export type FooterInfo = {
  copyright: string,
  year: number,
  name: string,
  content: string,
}
