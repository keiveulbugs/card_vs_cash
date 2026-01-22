use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;
mod utils;

// Top-Level pages
use crate::pages::chapter::ChapterPage;
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="SmartPay: What payment method fits you?"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-1280, initial-scale=1.0"/>

        <Router base="/card_vs_cash">
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/home") view=Home/>
                <Route path=path!("/chapter-2") view=|| view! { <ChapterPage chapter_num=2/> }/>
                <Route path=path!("/chapter-3") view=|| view! { <ChapterPage chapter_num=3/> }/>
                <Route path=path!("/chapter-4") view=|| view! { <ChapterPage chapter_num=4/> }/>
                <Route path=path!("/chapter-5") view=|| view! { <ChapterPage chapter_num=5/> }/>
                <Route path=path!("/chapter-6") view=|| view! { <ChapterPage chapter_num=6/> }/>
                <Route path=path!("/chapter-7") view=|| view! { <ChapterPage chapter_num=7/> }/>
            </Routes>
        </Router>
    }
}
