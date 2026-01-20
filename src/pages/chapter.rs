use crate::utils::{extract_title, get_chapter_content, MarkdownContent};
use leptos::prelude::*;

/// Chapter Page Component - used for all chapters except the home page
#[component]
pub fn ChapterPage(chapter_num: i32) -> impl IntoView {
    let content = get_chapter_content(chapter_num);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="error-container">
                    <h1>"Uh oh! Something went wrong!"</h1>
                    <p>"Errors: "</p>
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}

                    </ul>
                </div>
            }
        }>
            <Navbar/>

            <main class="main-content">
                <article class="chapter-card" id=format!("chapter-{}", chapter_num)>
                    <MarkdownContent content=content.to_string()/>
                </article>
            </main>

            <footer class="footer">
                <p>"Explore the evolving world of payments"</p>
            </footer>
        </ErrorBoundary>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-content">
                <ul class="nav-list">
                    <li>
                        <a href="/" class="nav-link">
                            "Why SmartPay?"
                        </a>
                    </li>
                    <li>
                        <a href="/chapter-2" class="nav-link">
                            "Current State"
                        </a>
                    </li>
                    <li>
                        <a href="/chapter-3" class="nav-link">
                            "Why Cash"
                        </a>
                    </li>
                    <li>
                        <a href="/chapter-4" class="nav-link">
                            "Pros and Cons"
                        </a>
                    </li>
                    <li>
                        <a href="/chapter-5" class="nav-link">
                            "Digital payment solutions"
                        </a>
                    </li>
                    <li>
                        <a href="/chapter-6" class="nav-link">
                            "Policy Initiatives and Petitions"
                        </a>
                    </li>
                    <li>
                        <a href="/chapter-7" class="nav-link">
                            "Calculator"
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
