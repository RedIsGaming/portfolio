<script lang="ts">
  import type { FooterInfo } from "./types";

  const footerInfo: FooterInfo[] = [
    {
      copyright: "Copyright ©",
      year: new Date().getFullYear(),
      name: "RedIsGaming",
      content: "All Rights Reserved",
    }
  ];
</script>

<footer class="w-full h-12 bg-white dark:bg-black shadow-lg fixed bottom-0">
  <section class="h-full text-black dark:text-white flex justify-center items-center mx-3 sm:mx-0">
    {#each footerInfo as footer}
      <p class="text-sm">{footer.copyright} {footer.year} {footer.name}. {footer.content}.</p>
    {/each}
  </section>
</footer>
