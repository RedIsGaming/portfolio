#![allow(non_snake_case)]

use leptos::{view, IntoView};

pub fn KeyPoint() -> impl IntoView {
    view! {
        <section class="bg-zinc-50 dark:bg-zinc-950 w-full font-mono text-black dark:text-white flex justify-center
            items-center flex-col py-24">
            <article>
                <h2>KeyPoint</h2>
                <br/>
                <p>KP page</p>
            </article>
        </section>
    }
}
