<script lang="ts">
  import type { About } from "../about";
  import Abouts from "../about";
  import { github, discord, redisgaming } from "../socials";
  import Redisgaming from "/src/assets/RedIsGaming.webp";

  const about_info: About[] = Abouts.new([
    {
      description: [
        `With already 4+ years in the software industry I have experience in many different languages like: Rust, C#, Svelte, TS 
        and more. I love to game in my free time, I'm active and I wonder and think about things that could inspire me. Because I
        want to stay healthy, i regularly go to the gym in my freetime. Not to forget, I'm a passionate and well driven developer 
        from the Netherlands.`
      ],
    },
    {
      description: [
        `Detailed, curious, specific and always ready for a challenge I keep pushing forward. Listening to inspiring ideas 
        fascinates me to the fullest and I'm independent and calm in nature. With my analytical capabilities, I always try to go 
        forward and search for solutions. You can get in touch with me via ${discord} and you might check out my projects on 
        ${github}. You can ask me anything!`
      ],
    },
  ]);

  const abouts: About[] = [...about_info];
  const abt: string = "About me";
</script>

<section class="portfolio custom-portfolio-section">
  <article class="portfolio-article">
    <h2 class="portfolio-h2">{abt}</h2>

    {#each abouts as abts, first}
      {#if first > 0}
        <img src="{Redisgaming}" alt="{redisgaming}" class="portfolio-redisgaming-img" />
      {/if}
      <p class="p-bigger">{abts.description}</p>
    {/each}
  </article>
</section>
