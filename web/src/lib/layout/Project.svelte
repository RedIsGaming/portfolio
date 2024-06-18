<script lang="ts">
  import Rust from "../../assets/technologies/rust.svg";
  import type { Project } from "../projects";
  import Projects from "../projects";
  import CSharp from "../../assets/technologies/csharp.svg";
  import Svelte from "../../assets/technologies/svelte.svg";

  const rust_colors: Project = Projects.new({ 
    name: "rust-colors",
    icon: {
      src: Rust,
      alt: "Rust-logo",
    },
    description: "A library for parsing ANSI Strings to colors.",
    keywords: ["rust", "library", "parsing", "ansi", "crates-io"],
    language: {
      name: "Rust",
      color: "bg-orange-200",
    },
    license: "MIT License",
    url: "https://github.com/RedIsGaming/rust-colors",
    target: "_blank",
  });

  const rlsyscli: Project = Projects.new({ 
    name: "rlsyscli",
    icon: {
      src: CSharp,
      alt: "CSharp-logo",
    },
    description: "A functional CLI to show your system info.",
    keywords: ["linux", "shell", "csharp", "storage", "display", "disk-space", "processes"],
    language: {
      name: "C#",
      color: "bg-green-600",
    },
    license: "Apache-2.0 license",
    url: "https://github.com/RedIsGaming/rlsyscli",
    target: "_blank",
  });

  const portfolio: Project = Projects.new({ 
    name: "portfolio",
    icon: {
      src: Svelte,
      alt: "Svelte-logo",
    },
    description: "My brand new personal portfolio written in Svelte, Typescript(TS) and TailWindCSS.",
    keywords: ["portfolio", "typescript", "spa", "web", "svelte", "tailwindcss", "redisgaming"],
    language: {
      name: "Svelte",
      color: "bg-orange-600",
    },
    license: "MIT License",
    url: "#projects",
    target: "_self",
  });

  const projects: Project[] = [rust_colors, rlsyscli, portfolio];
  const proj: string = "Projects";
  const more: string = "You can view more on Github";
</script>

<section class="portfolio portfolio-section">
  <article class="portfolio-article">
    <h2 class="portfolio-h2">{proj}</h2>

    {#each projects as projs}
      <a href="{projs.url}" target="{projs.target}">
        <article class="portfolio-project-article">
          <div class="portfolio-project-div">
            <h3 class="portfolio-h3">{projs.name}</h3>

            {#if projs.language.name.startsWith("Rust")}
              <img 
                class="rust-invert portfolio-project-img" 
                src="{projs.icon.src}" 
                alt={projs.icon.alt} 
              />
              {:else}
                <img 
                  class="portfolio-project-img" 
                  src="{projs.icon.src}" 
                  alt={projs.icon.alt} 
                />
            {/if}
          </div>

          <p class="my-2 gray-text p">{projs.description}</p>
          <p class="inline-block mt-2 mb-8 font-black p">{projs.keywords.join(" ")}</p>
          
          <div class="portfolio-project-div">
            <div class="items-center portfolio-project-div">
              <span class="{projs.language.color} w-3 h-3 rounded-full mr-1 sm:w-4 sm:h-4"></span>
              <p class="p">{projs.language.name}</p>
            </div>
            <p class="gray-text p">{projs.license}</p>
          </div>
          
        </article>
      </a>
    {/each}
    
    <article class="portfolio">
      <a 
        href="https://github.com/RedIsGaming" 
        class="flex justify-center w-full h-16 bg-blue-600 rounded-2xl sm:w-1/2 md:w-2/5 lg:w-1/3">
        <button class="text-xl font-semibold text-white">{more}</button>
      </a>
    </article>
  </article>
</section>
