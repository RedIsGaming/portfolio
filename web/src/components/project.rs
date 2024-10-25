#![allow(non_snake_case)]

use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn Project() -> impl IntoView {
    view! {
        <Title text="Project" />
        <p class="text-black text-4xl">"Hello, Project"</p>
    }
}
