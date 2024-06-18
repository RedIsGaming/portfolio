<script lang="ts">
  import type { Footer } from "../footer";
  import { redisgaming } from "../socials";
  import Footers from "../footer";

  const footer_info: Footer = Footers.new({
    copyright: "Copyright ©",
    year: new Date().getFullYear(),
    name: redisgaming,
    content: "All Rights Reserved",
  });

  const footers: Footer[] = [footer_info];
</script>

{#each footers as foot}
  <footer class="relative -bottom-24 w-full h-12 bg-white border-t-2 shadow-lg dark:bg-black" style="border-color: #0561D1;">
    <section class="flex justify-center items-center mx-3 h-full text-black sm:mx-0 dark:text-white">
      <p class="text-sm sm:text-base">{foot.copyright} {foot.year} {foot.name}. {foot.content}.</p>
    </section>
  </footer>
{/each}
