import { IconSource } from "svelte-hero-icons";

export type Icon = {
  name: string,
  href?: string,
  src: IconSource | string,
  size?: number,
};

export type FooterInfo = {
  copyright: string,
  year: number,
  name: string,
  content: string,
};

export type RedIsGaming = {
  name: string,
  src?: string,
  description: string[],
};

export type Technologies = {
  name: string,
  src: string,
  url: string,
};

export type ContactInfo = {
  description: string[],
  platform: string,
  url: string,
};
