#![allow(non_snake_case)]

use leptos::{view, IntoView};
use leptos_router::A;

#[derive(Debug, Default, Clone)]
struct CollectionInfo {
    d: Vec<String>,
    title: Vec<String>,
    description: Vec<String>,
    match_exact: bool,
}

impl CollectionInfo {
    fn new() -> Self {
        Self {
            d: vec![
                "M13 9h-2V7h2m0 10h-2v-6h2m-1-9A10 10 0 0 0 2 12a10 10 0 0 0 10 10a10 10 0 0 0 10-10A10 10 0 0 0 12 2".to_owned(),
                "M19 19H5V8h14m0-5h-1V1h-2v2H8V1H6v2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m-2.47 8.06L15.47 10l-4.88 4.88l-2.12-2.12l-1.06 1.06L10.59 17z".to_owned(),
                "m20.083 10.5l1.202.721a.5.5 0 0 1 0 .858L12 17.649l-9.285-5.57a.5.5 0 0 1 0-.858l1.202-.721L12 15.35zm0 4.7l1.202.721a.5.5 0 0 1 0 .858l-8.77 5.262a1 1 0 0 1-1.03 0l-8.77-5.262a.5.5 0 0 1 0-.858l1.202-.721L12 20.05zM12.514 1.309l8.771 5.262a.5.5 0 0 1 0 .858L12 12.999L2.715 7.43a.5.5 0 0 1 0-.858l8.77-5.262a1 1 0 0 1 1.03 0".to_owned(),
                "M2 22a8 8 0 1 1 16 0zm8-9c-3.315 0-6-2.685-6-6s2.685-6 6-6s6 2.685 6 6s-2.685 6-6 6m10 4h4v2h-4zm-3-5h7v2h-7zm2-5h5v2h-5z".to_owned()
            ],
            title: vec![
                "about".to_owned(), 
                "project".to_owned(), 
                "stack".to_owned(), 
                "contact".to_owned()
            ], 
            description: vec![
                "You can get to know more about me by clicking the button below. I'm Red and I write software. With passion for this and over 4 years, I've gained more experience on the way. With my analytical skills, I solve problems. I create solutions that can fit into any software product. Do you want to know more about me? You can find it out below.".to_owned(), 
                "You can get to know more about me by clicking the button below. I'm Red and I write software. With passion for this and over 4 years, I've gained more experience on the way. With my analytical skills, I solve problems. I create solutions that can fit into any software product. Do you want to know more about me? You can find it out below.".to_owned(), 
                "You can get to know more about me by clicking the button below. I'm Red and I write software. With passion for this and over 4 years, I've gained more experience on the way. With my analytical skills, I solve problems. I create solutions that can fit into any software product. Do you want to know more about me? You can find it out below.".to_owned(), 
                "You can get to know more about me by clicking the button below. I'm Red and I write software. With passion for this and over 4 years, I've gained more experience on the way. With my analytical skills, I solve problems. I create solutions that can fit into any software product. Do you want to know more about me? You can find it out below.".to_owned()
            ],
            match_exact: true, 
        }
    }
}

pub fn Collection() -> impl IntoView {
    let collection_info = CollectionInfo::new();

    view! {
        <section class="bg-white dark:bg-black w-full font-mono text-black dark:text-white flex justify-center
            items-center flex-col py-24 px-3 lg:px-32 text-center">
            <h2 class="text-5xl sm:text-6xl font-bold pb-6">Collections</h2>
            <article class="grid grid-cols md:grid-cols-2 w-full gap-5 content-center">
            {
                collection_info.d.iter().zip(collection_info.title).zip(collection_info.description).map(|((a, b), c)| {
                    let url = format!("/{}", b);

                    view! {
                        <article class="w-full p-10 bg-slate-100 dark:bg-slate-900 rounded-3xl">
                            <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" class="w-full">
                                <path fill="white" d=a />
                            </svg>
                            <h3 class="text-4xl pt-6">{b.to_uppercase()}</h3>
                            <p class="py-8 text-lg">{c}</p>
                            <A href=url exact=collection_info.match_exact>
                                <button class="bg-red-600 w-full h-12 rounded-2xl text-2xl font-semibold text-white scale-100 
                                    hover:scale-105 duration-300 ease-out hover:ease-in">
                                    Show more
                                </button>
                            </A>
                        </article>
                    }
                }).collect::<Vec<_>>()
            }
            </article>
        </section>
    }
}
