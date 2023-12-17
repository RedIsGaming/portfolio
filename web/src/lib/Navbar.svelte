<script lang="ts">
  import { Icon, CodeBracket, Home, Moon, Sun } from "svelte-hero-icons";
  import type { CustomIcon } from "./types";
  
  function darkmode(): boolean {
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  }

  function icon(customIcon: CustomIcon): CustomIcon {
    return { ...customIcon, href: "/", size: 48, solid: true };
  }

  const logo: CustomIcon[] = [{ src: CodeBracket }];
  const home: CustomIcon[] = [{ src: Home }];
  const other: CustomIcon[] = [
    {
      name: "github",
      href: "https://github.com/RedIsGaming/",
      src: "/src/assets/github.svg",
    },

    {
      name: "discord",
      href: "https://discordapp.com/users/724341024415285319",
      src: "/src/assets/discord.svg", 
    },
  ];
  
  export let logos = logo.map(icon);
  export let homes = home.map(icon);

  $: darkmode();
</script>

<header class="w-full h-24 bg-white dark:bg-black shadow-lg fixed">
  <nav class="h-full">
    <article class="h-full mx-24 text-black dark:text-white flex justify-between items-center">
      {#each logos as icon}
      <div>
        <a href="{icon.href}" class="flex items-center">
          <Icon src="{icon.src}" size="{icon.size?.toString()}" solid={icon.solid} />
          <p class="ml-2">RedIsGaming</p>
        </a>
      </div>
      {/each}
      {#each homes as icon}
        <div>
          <a href="{icon.href}">
            <Icon src="{icon.src}" size="{icon.size?.toString()}" solid={icon.solid} />
          </a>
        </div>
      {/each}
      <div class="flex">
        {#if darkmode()}
          <a href="/"><Icon src="{Moon}" size="48" solid /></a>
        {:else}
          <a href="/"><Icon src="{Sun}" size="48" solid /></a>
        {/if}
          <a href="{other[0].href}" target="_blank">
            <img src="{other[0].src}" alt="{other[0].name}" class="invert-0 dark:invert w-12 ml-4" />
          </a>
          <a href="{other[1].href}" target="_blank">
            <img src="{other[1].src}" alt="{other[1].name}" class="invert-0 dark:invert w-12 ml-4" />
          </a>
      </div>
    </article>
  </nav>
</header>
