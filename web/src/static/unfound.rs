use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn UnFound() -> impl IntoView {
    let title = String::from("404").to_uppercase();
    let description = String::from("The page couldn't be found.").to_uppercase();

    view! {
        <Title text="404 Page not found" />
        <section class="bg-zinc-50 dark:bg-zinc-950 w-full h-[50.5vh] font-mono text-black dark:text-white flex justify-center 
            items-center flex-col">
            <h1 class="text-6xl sm:text-8xl font-bold w-auto">{title}</h1>
            <br/>
            <p class="text-lg sm:text-2xl font-normal w-auto">{description}</p>
        </section>
    }
}
