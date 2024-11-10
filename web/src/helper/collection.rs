use leptos::{component, view, IntoView};
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
                "M2 22a8 8 0 1 1 16 0zm8-9c-3.315 0-6-2.685-6-6s2.685-6 6-6s6 2.685 6 6s-2.685 6-6 6m10 4h4v2h-4zm-3-5h7v2h-7zm2-5h5v2h-5z".to_owned(),
                "M24 0v24H0V0zM12.593 23.258l-.011.002l-.071.035l-.02.004l-.014-.004l-.071-.035q-.016-.005-.024.005l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427q-.004-.016-.017-.018m.265-.113l-.013.002l-.185.093l-.01.01l-.003.011l.018.43l.005.012l.008.007l.201.093q.019.005.029-.008l.004-.014l-.034-.614q-.005-.019-.02-.022m-.715.002a.02.02 0 0 0-.027.006l-.006.014l-.034.614q.001.018.017.024l.015-.002l.201-.093l.01-.008l.004-.011l.017-.43l-.003-.012l-.01-.01z".to_owned()
            ],
            title: vec!["about".to_owned(), "project".to_owned(), "stack".to_owned(), "contact".to_owned()], 
            description: vec![
                "I'm Red and I write software. In the current time, I've gained experience along the way. With my technical and solving skills, I solve problems. I create solutions that can fit into any software.".to_owned(), 
                "I've done a lot of projects in recent years. From frontend to backend and CLI's, Embedded, ORM's, Webapps, Backend systems and more. I'm passionate to learn more about certain topics.".to_owned(), 
                "Different programming languages serve their own purpose(s). With the time being, I've experienced and played with many different languages, tools and frameworks. Like Rust, TS, C# and more.".to_owned(), 
                "There is a lot to tell about this field and much opportunity. I'm always ready to have a future conversation. You can ask me anything. From projects, to stack choices or something else.".to_owned()
            ],
            match_exact: true, 
        }
    }
}

#[component]
pub fn Collection() -> impl IntoView {
    let collection_info = CollectionInfo::new();
    let title = String::from("Know more").to_uppercase();

    view! {
        <section class="section-nodefault flex-col text-center">
            <h2 class="heading-first pb-10">{title}</h2>
            <article class="grid grid-cols sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 w-full gap-6 content-center">
            {
                collection_info.d.iter().zip(collection_info.title).zip(collection_info.description).map(|((a, b), c)| {
                    let url = format!("/{}", b);

                    view! {
                        <article class="w-full p-3 rounded-3xl">
                            <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" class="w-full 
                                fill-current text-black dark:text-white">
                                <path d=a />
                            </svg>
                            <h3 class="heading-third pt-6">{b.to_uppercase()}</h3>
                            <p class="py-8 text-xl text-neutral-600 dark:text-neutral-400">{c}</p>
                            <A href=url exact=collection_info.match_exact>
                                <button class="bg-red-600 w-full h-14 rounded-2xl text-3xl text-white scale-100 
                                    hover:scale-105 duration-300 ease-out hover:ease-in">
                                    Read more
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
