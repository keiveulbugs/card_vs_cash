use crate::utils::{extract_title, get_chapter_content, MarkdownContent};
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
                            <MarkdownContent content=description/>
                        </div>
                    </header>
                }
            }
            <nav class="navbar">
                <div class="navbar-content">
                    <ul class="nav-list">
                        <li>
                            <a href="#chapter-1" class="nav-link">
                                "Introduction"
                            </a>
                        </li>
                        <li>
                            <a href="#chapter-2" class="nav-link">
                                "Historical Context"
                            </a>
                        </li>
                        <li>
                            <a href="#chapter-3" class="nav-link">
                                "Advantages of Cash"
                            </a>
                        </li>
                        <li>
                            <a href="#chapter-4" class="nav-link">
                                "Advantages of Cards"
                            </a>
                        </li>
                        <li>
                            <a href="#chapter-5" class="nav-link">
                                "Security"
                            </a>
                        </li>
                        <li>
                            <a href="#chapter-6" class="nav-link">
                                "Environment"
                            </a>
                        </li>
                        <li>
                            <a href="#chapter-7" class="nav-link">
                                "Future"
                            </a>
                        </li>
                    </ul>
                </div>
            </nav> <main class="main-content">
                <div class="chapters-container">
                    <Chapter chapter_num=1/>
                    <Chapter chapter_num=2/>
                    <Chapter chapter_num=3/>
                    <Chapter chapter_num=4/>
                    <Chapter chapter_num=5/>
                    <Chapter chapter_num=6/>
                    <Chapter chapter_num=7/>
                </div>
            </main> <footer class="footer">
                <p>"Explore the evolving world of payments"</p>
            </footer>
        </ErrorBoundary>
    }
}

#[component]
fn Chapter(chapter_num: i32) -> impl IntoView {
    let content = get_chapter_content(chapter_num);
    let chapter_id = format!("chapter-{}", chapter_num);

    view! {
        <article class="chapter-card" id=chapter_id>
            <MarkdownContent content=content.to_string()/>
        </article>
    }
}
