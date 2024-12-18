use leptos::{component, view, IntoView};
use leptos_router::A;

#[derive(Debug, Default, Clone)]
struct HeaderInfo {
    url: Vec<String>,
    match_exact: bool,
    name: Vec<String>,
}

impl HeaderInfo {
    fn new(url: Vec<String>, match_exact: bool, name: Vec<String>) -> Self {
        Self {
            url,
            match_exact,
            name,
        }
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let header_info = HeaderInfo::new(
        vec![String::from("/about"), String::from("/project"), String::from("/stack"), String::from("/contact")], 
        true, 
        vec![String::from("About"), String::from("Project"), String::from("Stack"), String::from("Contact")]
    );

    view! {
        <nav class="bg-white dark:bg-black text-black dark:text-white w-full sm:flex sm:items-center sm:justify-between py-4 
            sm:py-8 px-3 lg:px-16 font-semi-bold flex-col sm:flex-row text-center shadow-lg dark:shadow-none">
            <article class="text-6xl font-fantasy  w-full sm:w-auto pb-6 sm:p-0">
                <A href="/" exact=header_info.match_exact>Red</A>
            </article>
            <article class="text-xl sm:text-2xl font-fantasy w-full sm:w-auto text-center">
            {
                header_info.url.into_iter().zip(header_info.name).map(|(x, y)| {
                    view! {
                        <A href=x exact=header_info.match_exact class="ml-3 sm:ml-9">{y}</A>
                    }
                }).collect::<Vec<_>>()
            }
            </article>
        </nav>
    }
}
