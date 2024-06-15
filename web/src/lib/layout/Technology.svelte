<script lang="ts">
  import Rust from "../../assets/technologies/rust.svg";
  import CSharp from "../../assets/technologies/csharp.svg";
  import PHP from "../../assets/technologies/php.svg";
  import TypeScript from "../../assets/technologies/typescript.svg";
  import Svelte from "../../assets/technologies/svelte.svg";
  import Tailwindcss from "../../assets/technologies/tailwindcss.svg";

  import JavaScript from "../../assets/technologies/javascript.svg";
  import C from "../../assets/technologies/c.svg";
  import Python from "../../assets/technologies/python.svg";
  import HTML from "../../assets/technologies/html5.svg";
  import CSS from "../../assets/technologies/css3.svg";
  import MySQL from "../../assets/technologies/mysql.svg";

  import type { Technology } from "../technology";
  import Technologies from "../technology";

  const backend: Technology[] = Technologies.new([
    {
      name: "Rust",
      src: Rust,
    },
    {
      name: "MySQL",
      src: MySQL,
    },
    {
      name: "C#",
      src: CSharp,
    },
    {
      name: "C",
      src: C,
    },
    {
      name: "PHP",
      src: PHP,
    },
    {
      name: "Python",
      src: Python,
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
    {
      name: "HTML",
      src: HTML,
    },
    {
      name: "CSS",
      src: CSS,
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
        <div class="w-full flex justify-center">
          <div class="portfolio-technology-div">
            {#if techs.name.startsWith("Rust")}
              <img class="rust-invert w-20 md:w-28" src="{techs.src}" alt="{techs.name}" />
              
              {:else if techs.name.endsWith("Script")}
                <img class="w-20 md:w-28" src="{techs.src}" alt="{techs.name}" />
              {:else if techs.name.startsWith("Python") || techs.name.startsWith("MySQL")}
                <img class="w-{20 * 2} md:w-{28 * 2}" src="{techs.src}" alt="{techs.name}" />
              {:else}
                <img class="w-20 md:w-28" src="{techs.src}" alt="{techs.name}" />
            {/if}
          </div>
        </div>
      {/each}
    </article>
  </article>
</section>
