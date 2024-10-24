#![allow(non_snake_case)]

use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <article class="bg-white dark:bg-black text-black dark:text-white w-full h-64 text-4xl font-mono font-semibold text-center">
            <p>"Hello, Home"</p>
        </article>
    }
}
