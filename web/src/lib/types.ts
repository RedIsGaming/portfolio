import { IconSource } from "svelte-hero-icons";

export type CustomIcon = {
  name?: string,
  href?: string,
  src: IconSource | string,
  size?: number,
  solid?: boolean,
};
