#![allow(non_snake_case)]

use crate::helper::{banner, collection, keypoint};
use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <banner::Banner />
        <collection::Collection />
        <keypoint::KeyPoint />
    }
}
