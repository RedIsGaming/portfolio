<script lang="ts">
  import { Icon, CodeBracket, Home, Moon, Sun } from "svelte-hero-icons";
  import type { CustomIcon } from "./types";
  import { lightmode, darkmode } from "./theme";

  function icon(customIcon: CustomIcon): CustomIcon {
    return { ...customIcon, href: "/", size: 48 };
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
  
  export let logos: CustomIcon[] = logo.map(icon);
  export let homes: CustomIcon[] = home.map(icon);

  $: darkmode();
</script>

<header class="w-full h-24 bg-white dark:bg-black shadow-lg">
  <nav class="h-full">
    <article class="h-full mx-3 text-black dark:text-white flex justify-between items-center lg:mx-24">
      {#each logos as icon}
      <div>
        <a href="{icon.href}" class="flex items-center">
          <Icon src="{icon.src}" size="{icon.size?.toString()}" solid />
          <p class="ml-2 text-base sm:text-2xl">RedIsGaming</p>
        </a>
      </div>
      {/each}
      {#each homes as icon}
        <div>
          <a href="{icon.href}" class="hidden sm:inline">
            <Icon src="{icon.src}" size="{icon.size?.toString()}" solid />
          </a>
        </div>
      {/each}
      <div class="flex">
        {#if darkmode()}
          <a class="w-8 sm:w-12">
            <button on:click={lightmode}><Icon src="{Moon}" solid /></button>
          </a>
        {:else}
          <a class="w-8 sm:w-12">
            <button on:click={darkmode}><Icon src="{Sun}" solid /></button>
          </a>
        {/if}
          <a href="{other[0].href}" target="_blank">
            <img src="{other[0].src}" alt="{other[0].name}" class="invert-0 dark:invert w-8 ml-2 sm:w-12 sm:ml-4" />
          </a>
          <a href="{other[1].href}" target="_blank">
            <img src="{other[1].src}" alt="{other[1].name}" class="invert-0 dark:invert w-8 ml-2 sm:w-12 sm:ml-4" />
          </a>
      </div>
    </article>
  </nav>
</header>
