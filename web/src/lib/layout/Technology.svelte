<script lang="ts">
  import Rust from "../../assets/technologies/rust.svg";
  import CSharp from "../../assets/technologies/csharp.svg";
  import PHP from "../../assets/technologies/php.svg";
  import TypeScript from "../../assets/technologies/typescript.svg";
  import Svelte from "../../assets/technologies/svelte.svg";
  import Tailwindcss from "../../assets/technologies/tailwindcss.svg";
  import JavaScript from "../../assets/technologies/javascript.svg";

  import type { Technology } from "../technology";
  import Technologies from "../technology";

  const backend: Technology[] = Technologies.new([
    {
      name: "Rust",
      src: Rust,
    },
    {
      name: "C#",
      src: CSharp,
    },
    {
      name: "PHP",
      src: PHP,
    },
    {
      name: "TypeScript",
      src: TypeScript,
    },
  ]);

  const frontend: Technology[] = Technologies.new([
    {
      name: "Svelte",
      src: Svelte,
    },
    {
      name: "Tailwindcss",
      src: Tailwindcss,
    },
    {
      name: "TypeScript",
      src: TypeScript,
    },
    {
      name: "JavaScript",
      src: JavaScript,
    },
  ]);

  let technologies: Technology[] = [...backend, ...frontend];
  let tech: string = "Technologies";
</script>

<section class="portfolio portfolio-section">
  <article class="portfolio-article">
    <h2 class="portfolio-h2">{tech}</h2>

    <article class="portfolio-technology-article"> 
      {#each technologies as techs}
        <div class="portfolio-technology-div">
          {#if techs.name.startsWith("Rust")}
            <img class="rust-invert" src="{techs.src}" alt="{techs.name}" width={64} />
            
            {:else if techs.name.endsWith("Script")}
              <img class="rounded-full" src="{techs.src}" alt="{techs.name}" width={64} />
            {:else}
              <img src="{techs.src}" alt="{techs.name}" width={64} />
          {/if}
        </div>
      {/each}
    </article>
  </article>
</section>
