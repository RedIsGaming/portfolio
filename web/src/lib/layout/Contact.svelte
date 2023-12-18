<script lang="ts">
  import type { ContactInfo } from "../types";
  
  export let github: string = "";
  export let discord: string = "";

  const contactInfo: ContactInfo[] = [
    {
      description: [
        `You can get in touch with me on ${discord}. Maybe we will have a great conversation right there. 
        My projects can be found on my ${github}. Be sure to check it out. I don't use other social media platforms, so you know.`
      ],
      platform: github,
      url: `https://${github.toLowerCase()}.com/RedIsGaming/`,
    },
    {
      description: [],
      platform: discord,
      url: `https://${discord.toLowerCase()}app.com/users/724341024415285319`,
    }
  ];
</script>

<section class="w-full bg-gray dark:bg-dark text-black dark:text-white flex justify-center">
  <article class="my-6 sm:my-12 mx-3 lg:mx-0" style="width: 1024px;">
    <h2 class="text-4xl sm:text-5xl">Contact</h2>
    {#each contactInfo as contact}
      <p class="my-5">{contact.description}</p>
      <a href="{contact.url}" target="_blank" class="text-2xl sm:text-3xl font-semibold" style="font-family: Lucida Console;">
        <button class="w-1/2 sm:w-1/3  md:w-1/4 h-14 sm:h-16 rounded-2xl" style="background-color: #0561D1;">
          {contact.platform}
        </button>
      </a>
    {/each}
  </article>
</section>
