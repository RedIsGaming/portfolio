use crate::helper::{banner, collection};
use leptos::{component, view, IntoView};
use leptos_meta::{Meta, Title};

#[component]
fn HomeSEO() -> impl IntoView {
    view! {
        <Title text="Red - Homepage" />
        <Meta 
            name="description"
            content="Red is a passionate software developer with heart for technology and language stacks that follow conventions" 
        />
        <Meta name="keywords" content="Red, Homepage, Software, Developer, Technology, Portfolio, Stacks, Conventions" />
        <Meta name="robots" content="index, follow" />
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <HomeSEO />
        <banner::Banner />
        <collection::Collection />
    }
}
