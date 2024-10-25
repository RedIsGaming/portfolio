#![allow(non_snake_case)]

use leptos::{view, IntoView};
use leptos_router::A;

#[derive(Debug, Default, Clone)]
struct CollectionSvgPath {
    d: Vec<String>,
}

impl CollectionSvgPath {
    fn new() -> Self {
        Self {
            d: vec![
                "M13 9h-2V7h2m0 10h-2v-6h2m-1-9A10 10 0 0 0 2 12a10 10 0 0 0 10 10a10 10 0 0 0 10-10A10 10 0 0 0 12 2".to_owned(),
                "M19 19H5V8h14m0-5h-1V1h-2v2H8V1H6v2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m-2.47 8.06L15.47 10l-4.88 4.88l-2.12-2.12l-1.06 1.06L10.59 17z".to_owned(),
                "m20.083 10.5l1.202.721a.5.5 0 0 1 0 .858L12 17.649l-9.285-5.57a.5.5 0 0 1 0-.858l1.202-.721L12 15.35zm0 4.7l1.202.721a.5.5 0 0 1 0 .858l-8.77 5.262a1 1 0 0 1-1.03 0l-8.77-5.262a.5.5 0 0 1 0-.858l1.202-.721L12 20.05zM12.514 1.309l8.771 5.262a.5.5 0 0 1 0 .858L12 12.999L2.715 7.43a.5.5 0 0 1 0-.858l8.77-5.262a1 1 0 0 1 1.03 0".to_owned(),
                "M2 22a8 8 0 1 1 16 0zm8-9c-3.315 0-6-2.685-6-6s2.685-6 6-6s6 2.685 6 6s-2.685 6-6 6m10 4h4v2h-4zm-3-5h7v2h-7zm2-5h5v2h-5z".to_owned()
            ]
        }
    }
}

#[derive(Debug, Default, Clone)]
struct CollectionInfo {
    title: Vec<String>,
    description: Vec<String>,
}

impl CollectionInfo {
    fn new() -> Self {
        Self {
            title: vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()], 
            description: vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()],
        }
    }
}

#[derive(Debug, Default)]
struct CollectionNavigate {
    url: Vec<String>,
    match_exact: bool,
    content: Vec<String>,
}

impl CollectionNavigate {
    fn new() -> Self {
        Self {
            url: vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()], 
            match_exact: true, 
            content: vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()],
        }
    }
}

pub fn Collection() -> impl IntoView {
    let collection_svgpath = CollectionSvgPath::new();
    let collection_info = CollectionInfo::new();
    let collection_navigate = CollectionNavigate::new();

    view! {
        <section class="bg-white dark:bg-black w-full font-mono text-black dark:text-white flex justify-center
            items-center flex-col py-24 px-3 lg:px-32 text-center">
            <h2 class="text-5xl sm:text-6xl font-bold pb-6">Collections</h2>
            <article class="grid grid-cols-2 w-full gap-5 content-center">
            {
                collection_svgpath.d.iter()
                    .zip(collection_info.title)
                    .zip(collection_info.description)
                    .zip(collection_navigate.url)
                    .zip(collection_navigate.content)
                    .map(|((((a, b), c), d), e)| {
                        view! {
                            <article class="w-full border-2 border-red-600 p-10">
                                <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" class="w-full">
                                    <path fill="white" d=a />
                                </svg>
                                <h3>{b}</h3>
                                <p>{c}</p>
                                <button><A href=d exact=collection_navigate.match_exact>{e}</A></button>
                            </article>
                        }
                    })
                    .collect::<Vec<_>>()
            }
            </article>
        </section>
    }
}
