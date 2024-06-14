import { IconSource } from "svelte-hero-icons";

export type Icon = {
  name: string,
  href?: string,
  src: IconSource | string,
  size?: number,
};
