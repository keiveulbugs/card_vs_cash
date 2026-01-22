use crate::utils::{extract_title, get_chapter_content, MarkdownContent, MarkdownContent2};
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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

            {
                let title_content = get_chapter_content(0);
                let title = extract_title(title_content);
                let description = title_content
                    .lines()
                    .skip_while(|line| line.starts_with('#'))
                    .collect::<Vec<_>>()
                    .join("\n");
                view! {
                    <header class="header">
                        <div class="header-content">
                            <h1 class="header-title">{title}</h1>
                            <MarkdownContent2 content=description/>
                        </div>
                    </header>
                }
            }
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
                                "Misconceptions"
                            </a>
                        </li>
                        <li>
                            <a href="/chapter-4" class="nav-link">
                                "Digital Payment Solutions"
                            </a>
                        </li>

                        <li>
                            <a href="/chapter-5" class="nav-link">
                                "Policy Initiatives"
                            </a>
                        </li>
                        <li>
                            <a href="/chapter-6" class="nav-link">
                                "PoS Calculator"
                            </a>
                        </li>
                        <li>
                            <a href="/chapter-7" class="nav-link">
                                "Who are we?"
                            </a>
                        </li>
                    </ul>
                </div>
            </nav> <main class="main-content">
                <article class="chapter-card" id="chapter-1">
                    <MarkdownContent content=get_chapter_content(1).to_string()/>
                </article>
            </main> <footer class="footer">
                <p>"Explore the evolving world of payments"</p>
            </footer>
        </ErrorBoundary>
    }
}
