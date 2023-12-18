<script lang="ts">
  import Navbar from "./lib/Navbar.svelte";
  import Footer from "./lib/Footer.svelte";
  import RedIsGaming from "./lib/RedIsGaming.svelte";
  import Technology from "./lib/Technology.svelte";

  export let reddy: string = "RedIsGaming";
</script>

<Navbar />
<main>
  <RedIsGaming redisgaming="{reddy}" />
  <Technology />
</main>
<Footer redisgaming="{reddy}" />
