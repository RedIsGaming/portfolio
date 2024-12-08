use leptos::{component, view, IntoView};
use leptos_meta::{Meta, Title};
use leptos_router::A;

#[component]
fn ContactSEO() -> impl IntoView {
    view! {
        <Title text="Red - Contact" />
        <Meta 
            name="description"
            content="Red like to talk about technology, stacks, programming languages and so much more. Click here to contact him" 
        />
        <Meta name="keywords" content="Red, Contact, Contact Info, Contact me" />
        <Meta name="robots" content="index, follow" />
    }
}

#[component]
pub fn Contact() -> impl IntoView {
    let text = String::from("Red").to_uppercase();

    view! {
        <ContactSEO />

        <section class="section-nodefault">
            <article class="w-full lg:w-9/12">
                <h1 class="text-5xl sm:text-6xl font-mono font-bold text-center">{"Contact".to_uppercase()}</h1>
                <article class="bg-slate-200 dark:bg-slate-800 mt-6 p-10 sm:py-20 rounded-3xl flex flex-col sm:flex-row 
                    ">
                    <div class="w-full sm:w-3/4">
                        <h2 class="text-4xl sm:text-5xl font-mono font-semibold">{"Contact Info".to_uppercase()}</h2>
                        <p class="mt-6 mb-3 font-mono text-neutral-700 dark:text-neutral-300 text-xl">
                            "We can talk about anything. stacks, programming languages, technology and more. You can reach out to me 
                            by sending me "
                            <A href="mailto:j.hofman-1@student.hhs.nl" class="text-red-600">an email</A>.
                        </p>
                        <div class="w-12 duration-300 scale-100 hover:scale-110 ease-out hover:ease-in">
                            <A href="mailto:j.hofman-1@student.hhs.nl">
                                <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24">
                                    <path fill="currentColor" d="M21 12.13c-.15.01-.29.06-.39.17l-1 1l2.05 2l1-1c.22-.21.22-.56 0-.77l-1.24-1.23a.56.56 0 0 0-.38-.17m-2 1.75L13 19.94V22h2.06l6.06-6.07M20 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h7v-.89l8.24-8.22c.47-.49 1.12-.76 1.8-.76c.34 0 .68.06 1 .19V6c0-1.12-.92-2-2.04-2m0 4l-8 5l-8-5V6l8 5l8-5"/>
                                </svg>
                            </A>
                        </div>
                    </div>
                    <aside class="w-full sm:w-1/4 mt-3 sm:mt-0 ml-0 sm:ml-4">
                        <figure class="flex justify-start sm:justify-end">
                            <img src="./assets/Red.webp" alt={text.clone()} class="rounded-full w-32 h-32 border-black 
                                dark:border-white border-2" 
                            />
                        </figure>
                    </aside>
                </article>
            </article>
        </section>
    }
}
