<script lang="ts">
  import Navbar from "./lib/layout/Navbar.svelte";
  import Footer from "./lib/layout/Footer.svelte";
  import RedIsGaming from "./lib/layout/RedIsGaming.svelte";
  import Technology from "./lib/layout/Technology.svelte";
  import Contact from "./lib/layout/Contact.svelte";
</script>

<Navbar />
<main class="top-24 relative">
  <RedIsGaming />
  <Technology />
  <Contact />
</main>
<Footer />
