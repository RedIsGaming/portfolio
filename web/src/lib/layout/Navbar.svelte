<script lang="ts">
  import { Icon } from "svelte-hero-icons";
  import { logos, homes, socials } from "../icons";
  import { reddy } from "../socials";
</script>

<header class="w-full h-24 bg-white dark:bg-black shadow-lg fixed z-10 border-b-2" style="border-color: #0561D1;">
  <nav class="h-full">
    <article class="h-full mx-3 text-black dark:text-white flex justify-between items-center lg:mx-24">
      {#each logos as icon}
        <div>
          <a href="{icon.href}" class="flex items-center">
            <Icon src="{icon.src}" size="{icon.size?.toString()}" solid />
            <p class="ml-2 text-2xl">{reddy}</p>
          </a>
        </div>
      {/each}
      {#each homes as icon}
        <div>
          <a href="{icon.href}" class="hidden sm:inline" aria-label="Return to RedIsGaming's portfolio">
            <Icon src="{icon.src}" size="{icon.size?.toString()}" solid />
          </a>
        </div>
      {/each}
      <article class="flex">
        {#each socials as icon}
          <div>
            <a href="{icon.href}" target="_blank">
              <img src="{icon.src}" alt="{icon.name}" class="invert-0 dark:invert w-8 ml-2 sm:w-12 sm:ml-4" />
            </a>
          </div>
        {/each}
      </article>
    </article>
  </nav>
</header>
