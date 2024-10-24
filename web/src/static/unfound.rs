#![allow(non_snake_case)]

use leptos::{component, view, IntoView};

#[derive(Debug, Default)]
struct UnFoundInfo(Vec<String>, Vec<String>);

impl UnFoundInfo {
    fn title(&self) -> Vec<String> {
        self.0.iter().map(|x| x.to_uppercase()).collect()
    }

    fn description(&self) -> Vec<String> {
        self.1.iter().map(|x| x.to_uppercase()).collect()
    }
}

#[component]
pub fn UnFound() -> impl IntoView {
    let unfound_info = UnFoundInfo(
        vec![String::from("404")], 
        vec![String::from("The page couldn't be found.")]
    );

    view! {
        <section class="bg-zinc-50 dark:bg-zinc-950 w-full h-[81vh] font-mono text-black dark:text-white flex justify-center 
            items-center flex-col">
            <h1 class="text-6xl sm:text-8xl font-bold w-auto">{UnFoundInfo::title(&unfound_info)}</h1>
            <br/>
            <p class="text-lg sm:text-2xl font-normal w-auto">{UnFoundInfo::description(&unfound_info)}</p>
        </section>
    }
}
