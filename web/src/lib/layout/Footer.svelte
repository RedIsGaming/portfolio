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
  <footer class="w-full h-12 bg-white dark:bg-black shadow-lg -bottom-24 border-t-2 relative" style="border-color: #0561D1;">
    <section class="h-full text-black dark:text-white flex justify-center items-center mx-3 sm:mx-0">
      <p class="text-sm sm:text-base">{foot.copyright} {foot.year} {foot.name}. {foot.content}.</p>
    </section>
  </footer>
{/each}
