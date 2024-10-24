#![allow(non_snake_case)]

use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <article class="bg-white dark:bg-black text-black dark:text-white w-full h-16 p-6 text-xl font-mono font-semibold
            text-center bottom-0 ">
            <p>"Copyright © 2024 RedIsGaming. All Rights Reserved."</p>
        </article>
    }
}
