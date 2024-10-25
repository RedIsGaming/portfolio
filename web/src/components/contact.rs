#![allow(non_snake_case)]

use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="Contact" />
        <p class="text-black text-4xl">"Hello, Contact"</p>
    }
}
