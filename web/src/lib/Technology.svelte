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
  <article class="my-6 mx-3 lg:mx-0" style="width: 1024px;">
    <h2 class="text-5xl">Technologies</h2>
    <h3 class="text-4xl mt-8">Backend</h3>
    <article class="mt-4 grid grid-cols-4 gap-3"> 
      {#each backend as tech}
        <div class="h-60 rounded-2xl flex justify-center items-center border-2" style="border-color: #1FD15A;">
          <a href="{tech.url}" target="_blank" class="flex justify-center">
            <img src="{tech.src}" alt="{tech.name}" class="w-3/4" style="border-radius: 48px;" />
          </a>
        </div>
      {/each}
    </article>
    <h3 class="text-4xl mt-8">Frontend</h3>
    <article class="mt-4 grid grid-cols-4 gap-3"> 
      {#each frontend as tech}
        <div class="h-60 rounded-2xl flex justify-center items-center border-2" style="border-color: #1FD15A;">
          <a href="{tech.url}" target="_blank" class="flex justify-center">
            <img src="{tech.src}" alt="{tech.name}" class="w-3/4" style="border-radius: 48px;" />
          </a>
        </div>
      {/each}
    </article>
  </article>
</section>
