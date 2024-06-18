<script lang="ts">
  import { Icon } from "svelte-hero-icons";
  import { logos, homes, socials } from "../icons";
  import { reddy } from "../socials";
</script>

<header class="fixed z-10 w-full h-24 bg-white border-b-2 shadow-lg dark:bg-black" style="border-color: #0561D1;">
  <nav class="h-full">
    <article class="flex justify-between items-center mx-3 h-full text-black lg:mx-24 dark:text-white">
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
              <img src="{icon.src}" alt="{icon.name}" class="ml-2 w-8 sm:ml-4 sm:w-12 invert-0 dark:invert" />
            </a>
          </div>
        {/each}
      </article>
    </article>
  </nav>
</header>
