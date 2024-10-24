#![allow(non_snake_case)]

use leptos::{component, mount_to_body, view, IntoView};

#[component]
fn App() -> impl IntoView {
    view! {
        <h1 class="text-black text-center text-4xl">"Hello, World!"</h1>
    }
}

fn main() {
    mount_to_body(App);
}
