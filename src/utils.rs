use leptos::prelude::*;

/// Extract the first heading from markdown content
pub fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line[2..].to_string())
        .unwrap_or_else(|| "Untitled".to_string())
}

/// Component to render markdown as HTML
#[component]
pub fn MarkdownContent(#[prop(into)] content: String) -> impl IntoView {
    // Simple markdown to HTML conversion for headers and paragraphs
    let html_content = content
        .lines()
        .map(|line| {
            if line.starts_with("# ") {
                format!("<h2>{}</h2>", &line[2..])
            } else if line.starts_with("## ") {
                format!("<h3>{}</h3>", &line[3..])
            } else if line.starts_with("### ") {
                format!("<h4>{}</h4>", &line[4..])
            } else if line.starts_with("- ") {
                format!("<li>{}</li>", &line[2..])
            } else if !line.is_empty() {
                format!("<p>{}</p>", line)
            } else {
                String::new()
            }
        })
        .collect::<String>();

    view! { <div class="markdown-content" inner_html=html_content></div> }
}

/// Get markdown content by chapter number (embedded at compile time)
pub fn get_chapter_content(chapter_num: i32) -> &'static str {
    match chapter_num {
        0 => include_str!("../public/chapters/title.md"),
        1 => include_str!("../public/chapters/chapter1.md"),
        2 => include_str!("../public/chapters/chapter2.md"),
        3 => include_str!("../public/chapters/chapter3.md"),
        4 => include_str!("../public/chapters/chapter4.md"),
        5 => include_str!("../public/chapters/chapter5.md"),
        6 => include_str!("../public/chapters/chapter6.md"),
        7 => include_str!("../public/chapters/chapter7.md"),
        _ => "Chapter not found",
    }
}
