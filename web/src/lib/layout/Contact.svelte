<script lang="ts">
  import type { Contact } from "../types";
  import { github, discord } from "../socials";
  import { socials } from "../icons";

  const contact: Contact[] = [
    {
      description: [
        `You can get in touch with me on ${discord}. Maybe we will have a great conversation right there. 
        My projects can be found on my ${github}. Be sure to check it out. I don't use other social media platforms, so you know.`
      ],
    },
  ];
</script>

<section class="w-full bg-gray dark:bg-dark text-black dark:text-white flex justify-center">
  <article class="my-6 sm:my-12 mx-3 lg:mx-0" style="width: 1024px;">
    <h2 class="text-4xl sm:text-5xl">Contact</h2>
    {#each contact as contacts}
      <p class="my-5 sm:text-xl text-lg">{contacts.description}</p>
    {/each}
    <article class="flex">
      {#each socials as icon}
        <div>
          <a href="{icon.href}" target="_blank">
            <img src="{icon.src}" alt="{icon.name}" class="invert-0 dark:invert w-8 mr-2 sm:w-12 sm:mr-4" />
          </a>
        </div>
      {/each}
    </article>
  </article>
</section>
