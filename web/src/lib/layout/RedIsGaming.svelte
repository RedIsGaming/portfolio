<script lang="ts">
  import type { RedIsGaming } from "../types";
  import { redisgaming, github, discord, reddy } from "../socials";
  import Redisgaming from "/src/assets/RedIsGaming.svg";

  const redIsGaming: RedIsGaming[] = [
    {
      name: redisgaming,
      src: Redisgaming,
      description: [
        `Welcome my friend! I'm ${redisgaming} and I'm a developer from the Netherlands. Feel free to call me ${reddy} as well. 
        I actually am a backend preffered developer, but I don't mind doing frontend as well. Based on the planet earth, you 
        can check more about me on this website.`,
      ],
    },
    {
      name: "",
      description: [
        `Feel free to check my ${github} and ${discord} as well. This website as portfolio is also linked on my ${github}. You can 
        contact me via ${discord}. I'm always open for a conversation about literally everything. This project is made in Svelte 
        with Typescript and TailwindCSS. Build with Yarn and Vite.`,
      ],
    },
  ];
</script>

<section class="bg-gray w-full flex justify-center text-black dark:bg-dark dark:text-white">
  <article class="my-6 sm:my-12 mx-3 lg:mx-0 text-lg sm:text-xl" style="width: 1024px;">
    {#each redIsGaming as me}
      <h1 class="text-5xl sm:text-6xl">{me.name}</h1>
      <img src="{me.src}" alt="{me.name}" class="border-2 rounded-full w-32 mt-5" style="border-color: #0561D1;" />
      <p class="mt-5">{me.description}</p>
    {/each}
  </article>
</section>
