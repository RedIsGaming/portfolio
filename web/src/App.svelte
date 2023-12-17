<script lang="ts">
  import Navbar from "./lib/Navbar.svelte";
  import Footer from "./lib/Footer.svelte";
  import RedIsGaming from "./lib/RedIsGaming.svelte";

  export let reddy: string = "RedIsGaming";
</script>

<Navbar />
<main>
  <RedIsGaming redisgaming="{reddy}" />
</main>
<Footer redisgaming="{reddy}" />
