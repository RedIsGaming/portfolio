#![allow(unused_imports)]
use leptos::{component, create_effect, create_node_ref, create_render_effect, html::{video, Video}, view, IntoView};

#[component]
pub fn VideoControls() -> impl IntoView {
    let noderef = create_node_ref::<Video>();
    let time_value = 11.4;
    
    create_render_effect(move |_| {
        if let Some(video) = noderef.get() {
            video.set_muted(true);
            video.set_current_time(time_value);
        }
    });

    view! {
        <video autoplay loop playsinline disablePictureInPicture="true" class="bg-cover w-full h-[61vh] bg-center object-cover 
            sm:bg-center sm:object-center invisible sm:visible" 
            node_ref={noderef}>
            <source src="./assets/Banner.mp4" type="video/mp4" />
        </video>
    }
}
