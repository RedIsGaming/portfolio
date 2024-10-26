use leptos::{component, view, IntoView};
use crate::helper::mute_audio::VideoControls;

#[component]
pub fn Banner() -> impl IntoView {
    let title = String::from("Red").to_uppercase();

    view! {
        <section class="relative">
            <VideoControls />
            <article class="w-full font-mono text-white flex justify-center items-center flex-col py-32 text-center absolute 
                inset-0">
                <img src="./assets/Red.webp" class="border-red-600 border-2 rounded-full w-28 sm:w-32 h-28 sm:h-32 flex
                    justify-center items-center bg-neutral-950" 
                    alt={title.clone()} 
                />
                <h1 class="text-7xl sm:text-8xl md:text-9xl font-bold w-auto pt-4">{title}</h1>
                <p class="text-2xl text-neutral-400">A Software Developer</p>
            </article>
        </section>
    }
}
