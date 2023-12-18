<script lang="ts">
  import { techs } from "../technologies";
</script>

<section class="w-full bg-white dark:bg-black flex justify-center">
  <article class="my-6 sm:my-12 mx-3 lg:mx-0" style="width: 1024px;">
    <h2 class="text-4xl sm:text-5xl text-black dark:text-white">Technologies</h2>
    <h3 class="text-3xl sm:text-4xl mt-4 sm:mt-8 text-black dark:text-white">Backend && Frontend</h3>

    <article class="mt-4 grid grid-cols-2 sm:grid-cols-3 gap-6 sm:gap-3 md:grid-cols-4"> 
      {#each techs as tech}
        <div class="h-64 rounded-2xl flex justify-center items-center border-2 lg:h-60 md:h-48" style="border-color: #1FD15A;">
          <a href="{tech.url}" target="_blank" class="flex justify-center text-white">
            <img src="{tech.src}" alt="{tech.name}" class="w-7/12 lg:w-3/4 md:w-8/12" style="border-radius: 48px;" />
          </a>
        </div>
      {/each}
    </article>
  </article>
</section>
