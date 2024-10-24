#![allow(non_snake_case)]

use leptos::{component, view, IntoView};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <p class="text-black text-4xl">"Hello, About"</p>
    }
}
