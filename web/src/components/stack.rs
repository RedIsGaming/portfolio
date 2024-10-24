#![allow(non_snake_case)]

use leptos::{component, view, IntoView};

#[component]
pub fn Stack() -> impl IntoView {
    view! {
        <p class="text-black text-4xl">"Hello, Stack"</p>
    }
}
