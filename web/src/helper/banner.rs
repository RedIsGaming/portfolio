#![allow(non_snake_case)]

use leptos::{view, IntoView};

pub fn Banner() -> impl IntoView {
    let title = "Red".to_owned().to_uppercase();

    view! {
        <section class="w-full font-mono text-black dark:text-white flex justify-center items-center flex-col px-24 py-32 
            bg-zinc-50 dark:bg-zinc-950 text-center">
            <h1 class="text-8xl sm:text-9xl font-bold w-auto">{title}</h1>
            <p class="text-2xl text-neutral-600 dark:text-neutral-400">A Software Developer</p>
        </section>
    }
}
