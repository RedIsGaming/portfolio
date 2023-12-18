<script lang="ts">
  import Navbar from "./lib/Navbar.svelte";
  import Footer from "./lib/Footer.svelte";
  import RedIsGaming from "./lib/RedIsGaming.svelte";
  import Technology from "./lib/Technology.svelte";
  import Contact from "./lib/Contact.svelte";

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
