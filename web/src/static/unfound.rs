use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn UnFound() -> impl IntoView {
    let title = String::from("404").to_uppercase();
    let description = String::from("The page couldn't be found.").to_uppercase();

    view! {
        <Title text="404 Page not found" />
        <section class="section-nodefault flex-col h-[44vh] text-center">
            <h1 class="text-6xl sm:text-8xl font-mono font-bold w-auto">{title}</h1>
            <br/>
            <p class="text-lg sm:text-2xl w-auto">{description}</p>
        </section>
    }
}
