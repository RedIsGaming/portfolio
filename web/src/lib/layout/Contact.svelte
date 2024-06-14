<script lang="ts">
  import type { Contact } from "../contact";
  import { github, discord } from "../socials";
  import { socials } from "../icons";
  import Contacts from "../contact";
  
  const contact_info: Contact = Contacts.new({
    description: [
      `You can get in touch with me on ${discord}. Maybe we will have a great conversation right there. 
      My projects can be found on my ${github}. Be sure to check it out. I don't use other social media platforms, so you know.`
    ],
  });

  const contacts: Contact[] = [contact_info];
  let cont: string = "Contact";
</script>

<section class="portfolio custom-portfolio-section">
  <article class="portfolio-article">
    <h2 class="portfolio-h2">{cont}</h2>

    {#each contacts as conts}
      <p class="p-bigger">{conts.description}</p>
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
