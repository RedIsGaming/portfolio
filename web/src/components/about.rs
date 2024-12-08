use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />

        <section class="section-nodefault h-[44vh] text-center">
            <article class="w-full">
                <h1 class="heading-first">{"About coming soon...".to_uppercase()}</h1>
                <br />
                <p class="text-lg sm:text-2xl font-mono">{"This page exist, but is currently under construction."}</p>
            </article>
        </section>
    }
}
