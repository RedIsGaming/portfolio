#![allow(non_snake_case)]

use leptos::{component, view, IntoView};
use leptos_router::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav class="bg-white dark:bg-black text-black dark:text-white w-full h-32 sm:flex sm:items-center p-6 
            font-semibold sm:flex-row fixed relative">
            <article class="text-6xl font-mono w-full">
                <A href="/" exact=true>Red</A>
            </article>
            <article class="text-3xl font-mono w-full sm:flex sm:justify-end">
                <A href="/about" exact=true class="ml-8">About</A>
                <A href="/project" exact=true class="ml-8">Project</A>
                <A href="/stack" exact=true class="ml-8">Stack</A>
                <A href="/contact" exact=true class="ml-8">Contact</A>
            </article>
        </nav>
    }
}
