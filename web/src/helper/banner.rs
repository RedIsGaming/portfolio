#![allow(non_snake_case)]

use leptos::{view, IntoView};

pub fn Banner() -> impl IntoView {
    view! {
        <section class="bg-zinc-50 dark:bg-zinc-950 w-full font-mono text-black dark:text-white flex justify-center 
            items-center flex-col py-24 bg-gradient-to-r from-black from-0% via-slate-800 via-50% to-red-900 to-100%">
            <article>
                <img src="/src/assets/Red.jpg" width=125 height=125 alt="Red" />
                <h1 class="text-6xl sm:text-8xl font-bold w-auto">Home</h1>
                <br/>
                <p class="text-lg sm:text-2xl font-normal w-auto">Home pagdsadsde</p>
            </article>
        </section>
    }
}
