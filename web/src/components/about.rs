#![allow(non_snake_case)]

use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />
        <p class="text-black text-4xl">"Hello, About"</p>
    }
}
