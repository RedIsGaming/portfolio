use crate::helper::{banner, collection};
use leptos::{component, view, IntoView};
use leptos_meta::{Meta, Title};

#[component]
fn HomeSEO() -> impl IntoView {
    view! {
        <Title text="Red Homepage" />
        <Meta charset="UTF-8" />
        <Meta content="width=device-width, initial-scale=1" name="viewport" />
        <Meta content="Red" name="author" />
        <Meta content="Red, Homepage, Software, Technology, Portfolio" name="keywords" />
        <Meta 
            content="Red is a passionate software developer with heart for technology and language stacks that follow conventions" 
            name="description" 
        />
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
