#![allow(non_snake_case)]

use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn Stack() -> impl IntoView {
    view! {
        <Title text="Stack" />
        <p class="text-black text-4xl">"Hello, Stack"</p>
    }
}
