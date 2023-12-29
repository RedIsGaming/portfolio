<script lang="ts">
  import type { Footer } from "../types";
  import { redisgaming } from "../socials";

  const footer: Footer = {
    copyright: "Copyright ©",
    year: new Date().getFullYear(),
    name: redisgaming,
    content: "All Rights Reserved",
  };
</script>

<footer class="w-full h-12 bg-white dark:bg-black shadow-lg -bottom-24 border-t-2 relative" style="border-color: #0561D1;">
  <section class="h-full text-black dark:text-white flex justify-center items-center mx-3 sm:mx-0">
    <p class="text-sm sm:text-base">{footer.copyright} {footer.year} {footer.name}. {footer.content}.</p>
  </section>
</footer>
