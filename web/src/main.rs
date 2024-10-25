#![allow(non_snake_case)]
#![allow(unused_imports)]

use web::{r#static::{header, footer, unfound}, components::{about, project, stack, contact}, home};
use leptos::{component, mount_to_body, view, IntoView};
use leptos_router::{Router, Routes, Route};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <header class="sticky w-full top-0">
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

            // <footer>
            //     <footer::Footer />
            // </footer>
        </Router>
    }
}

fn main() {
    mount_to_body(App);
}
