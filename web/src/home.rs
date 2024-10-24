#![allow(non_snake_case)]
#![allow(unused)]

use leptos::{component, view, IntoView};

#[derive(Debug, Default)]
struct Intro {
    title: String,
    description: String,
}

impl Intro {
    fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
        }
    }
}

fn HomeIntro() -> Intro {
    Intro::new(String::from(""), String::from(""))
}

fn HomeContent() -> Intro {
    Intro::new(String::from(""), String::from(""))
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section>
            <article class="bg-white dark:bg-black text-black dark:text-white w-full h-64 text-4xl font-mono font-semibold text-center">
                <p>"Hello, Home"</p>
            </article>
        </section>
    }
}
