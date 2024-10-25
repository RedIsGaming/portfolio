#![allow(non_snake_case)]

use leptos::{component, view, IntoView};
use chrono::{Datelike, Duration, Utc};

#[component]
pub fn Footer() -> impl IntoView {
    let datetime = Utc::now() + Duration::hours(2);

    view! {
        <article class="bg-zinc-50 dark:bg-zinc-950 text-black dark:text-white w-full h-16 p-6 text-lg font-mono 
            font-semibold text-center">
            <p>"Copyright © " {datetime.year()} " Red. All Rights Reserved."</p>
        </article>
    }
}
