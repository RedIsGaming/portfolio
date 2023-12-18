<script lang="ts">
  import Navbar from "./lib/layout/Navbar.svelte";
  import Footer from "./lib/layout/Footer.svelte";
  import RedIsGaming from "./lib/layout/RedIsGaming.svelte";
  import Technology from "./lib/layout/Technology.svelte";
  import Contact from "./lib/layout/Contact.svelte";

  export let reddy: string = "RedIsGaming";
  export let git: string = "Github";
  export let bonfire: string = "Discord";
</script>

<Navbar />
<main>
  <RedIsGaming redisgaming="{reddy}" />
  <Technology />
  <Contact github="{git}" discord="{bonfire}"  />
</main>
<Footer redisgaming="{reddy}" />
