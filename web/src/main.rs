#![allow(non_snake_case)]
#![allow(unused_imports)]

use leptos_meta::{provide_meta_context, Html};
use web::{r#static::{header, footer, unfound}, components::{about, project, stack, contact}, home, helper::socials};
use leptos::{component, mount_to_body, view, IntoView};
use leptos_router::{Router, Routes, Route};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" />
        <Router>
            <header class="sticky w-full top-0 z-10">
                <header::Header />
            </header>

            <main>
                <Routes>
                    <Route path="/" view=home::Home />
                    // <Route path="/about" view=about::About />
                    // <Route path="/project" view=project::Project />
                    // <Route path="/stack" view=stack::Stack />
                    // <Route path="/contact" view=contact::Contact />
                    <Route path="*/any" view=unfound::UnFound />
                </Routes>
            </main>

            <footer class="pb-12 bg-zinc-50 dark:bg-zinc-950">
                <socials::Socials />
                <footer::Footer />
            </footer>
        </Router>
    }
}

fn main() {
    mount_to_body(App);
}
