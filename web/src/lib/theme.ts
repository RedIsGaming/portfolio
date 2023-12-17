export function darkmode(): boolean {
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

export function lightmode(): boolean {
  return window.matchMedia("(prefers-color-scheme: light)").matches;
}
