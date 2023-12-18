<script lang="ts">
  import type { Technologies } from "./types";

  const backend: Technologies[] = [
    {
      name: "Rust",
      src: "/src/assets/rust.svg",
      url: "https://www.rust-lang.org/",
    },
    {
      name: "C#",
      src: "/src/assets/csharp.svg",
      url: "https://docs.microsoft.com/en-us/dotnet/csharp/",
    },
    {
      name: "PHP",
      src: "/src/assets/php.svg",
      url: "https://www.php.net/",
    },
    {
      name: "TypeScript",
      src: "/src/assets/typescript.svg",
      url: "https://www.typescriptlang.org/",
    },
  ];

  const frontend: Technologies[] = [
    {
      name: "Svelte",
      src: "/src/assets/svelte.svg",
      url: "https://svelte.dev/",
    },
    {
      name: "Tailwindcss",
      src: "/src/assets/tailwindcss.svg",
      url: "https://tailwindcss.com/",
    },
    {
      name: "TypeScript",
      src: "/src/assets/typescript.svg",
      url: "https://www.typescriptlang.org/",
    },
    {
      name: "JavaScript",
      src: "/src/assets/javascript.svg",
      url: "https://www.javascript.com/",
    },
  ];
</script>

<section class="w-full bg-white text-black dark:bg-black dark:text-white flex justify-center">
  <article class="my-6 sm:my-12 mx-3 lg:mx-0" style="width: 1024px;">
    <h2 class="text-4xl sm:text-5xl">Technologies</h2>
    <h3 class="text-3xl sm:text-4xl mt-4 sm:mt-8">Backend</h3>
    <article class="mt-4 grid grid-cols-2 sm:grid-cols-3 gap-6 sm:gap-3 md:grid-cols-4"> 
      {#each backend as tech}
        <div class="h-64 rounded-2xl flex justify-center items-center border-2 lg:h-60 md:h-48" style="border-color: #1FD15A;">
          <a href="{tech.url}" target="_blank" class="flex justify-center">
            <img src="{tech.src}" alt="{tech.name}" class="w-7/12 lg:w-3/4 md:w-8/12" style="border-radius: 48px;" />
          </a>
        </div>
      {/each}
    </article>
    <h3 class="text-3xl sm:text-4xl mt-4 sm:mt-8">Frontend</h3>
    <article class="mt-4 grid grid-cols-2 sm:grid-cols-3 gap-6 sm:gap-3 md:grid-cols-4"> 
      {#each frontend as tech}
        <div class="h-64 rounded-2xl flex justify-center items-center border-2 lg:h-60 md:h-48" style="border-color: #1FD15A;">
          <a href="{tech.url}" target="_blank" class="flex justify-center">
            <img src="{tech.src}" alt="{tech.name}" class="w-7/12 lg:w-3/4 md:w-8/12" style="border-radius: 48px;" />
          </a>
        </div>
      {/each}
    </article>
  </article>
</section>
