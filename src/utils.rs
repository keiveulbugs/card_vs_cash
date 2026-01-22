use leptos::prelude::*;

/// Extract the first heading from markdown content
pub fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line[2..].to_string())
        .unwrap_or_else(|| "Untitled".to_string())
}

use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, TagEnd};

#[component]
pub fn MarkdownContent2(#[prop(into)] content: String) -> impl IntoView {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);

    let mut custom_events = Vec::new();
    let mut image_count = 0;

    for event in parser {
        match event {
            // FIX 1: Use struct pattern matching with named fields { ... }
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                // Logic: Fix path
                let new_dest = fix_image_path(&dest_url);

                // Logic: Layout alignment
                let align_class = if image_count % 2 == 0 {
                    "left"
                } else {
                    "right"
                };
                image_count += 1;

                // Inject wrapper
                let wrapper_open = format!(r#"<div class="image-wrapper image-{}">"#, align_class);
                custom_events.push(Event::Html(wrapper_open.into()));

                // Push the image tag with the fixed URL
                // Note: We must reconstruct the Struct Variant here
                custom_events.push(Event::Start(Tag::Image {
                    link_type,
                    dest_url: new_dest.into(),
                    title,
                    id,
                }));
            }

            // FIX 2: Version 0.10+ uses 'TagEnd' enum for End events, not 'Tag'
            Event::End(TagEnd::Image) => {
                // Close the image
                custom_events.push(Event::End(TagEnd::Image));

                // Close the wrapper div
                custom_events.push(Event::Html("</div>".into()));
            }

            // Pass everything else through
            _ => custom_events.push(event),
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, custom_events.into_iter());

    view! { <div class="header-subtitle" inner_html=html_output></div> }
}

#[component]
pub fn MarkdownContent(#[prop(into)] content: String) -> impl IntoView {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(&content, options);

    let mut final_html_parts = Vec::new();
    let mut current_text_events = Vec::new();
    // Added a 4th field to the tuple for the description (alt text)
    let mut current_image_event: Option<(CowStr, CowStr, CowStr, String)> = None;
    let mut is_centered = false;
    let mut image_count = 0;

    // Trackers for image description parsing
    let mut is_collecting_alt = false;
    let mut current_alt_text = String::new();

    let mut flush_section = |image_data: Option<(CowStr, CowStr, CowStr, String)>,
                             text_events: Vec<Event>,
                             centered: bool| {
        let mut text_html = String::new();
        html::push_html(&mut text_html, text_events.into_iter());

        if text_html.trim().is_empty() && image_data.is_none() {
            return;
        }

        let align_class = if centered {
            "center-row"
        } else if image_count % 2 == 0 {
            "image-left"
        } else {
            "image-right"
        };

        // Capture the image and append the italicized description below it
        let img_html = if let Some((dest, title, _, alt)) = image_data {
            let fixed_dest = fix_image_path(&dest);

            // If description exists, create the italicized text block
            let description_html = if !alt.is_empty() {
                format!(r#"<p class="image-caption"><i>{}</i></p>"#, alt)
            } else {
                String::new()
            };

            format!(
                r#"<div class="image-col"><img src="{}" title="{}" alt="{}" />{}</div>"#,
                fixed_dest, title, alt, description_html
            )
        } else {
            String::new()
        };

        let section = format!(
            r#"<section class="content-row {}">
                {}
                <div class="text-col">{}</div>
               </section>"#,
            align_class, img_html, text_html
        );

        final_html_parts.push(section);

        if !centered && !img_html.is_empty() {
            image_count += 1;
        }
    };

    for event in parser {
        if is_collecting_alt {
            match event {
                Event::Text(t) => {
                    current_alt_text.push_str(&t);
                    continue; // Keep collecting until End tag
                }
                Event::End(TagEnd::Image) => {
                    is_collecting_alt = false;
                    // Now that we have the full alt text, update the current_image_event
                    if let Some(mut img) = current_image_event.take() {
                        img.3 = current_alt_text.clone();
                        current_image_event = Some(img);
                    }
                    continue;
                }
                _ => {} // Handle other nested tags if necessary
            }
        }

        match event {
            Event::Rule => {
                flush_section(current_image_event.take(), current_text_events, is_centered);
                current_text_events = Vec::new();
                is_centered = true;
            }

            Event::Start(Tag::Image {
                dest_url,
                title,
                id,
                ..
            }) => {
                // Flush existing content before starting a new image section
                flush_section(current_image_event.take(), current_text_events, is_centered);
                current_text_events = Vec::new();
                is_centered = false;

                // Initialize the image event with empty alt text, then start collecting
                current_image_event = Some((dest_url, title, id, String::new()));
                current_alt_text = String::new();
                is_collecting_alt = true;
            }

            _ => {
                current_text_events.push(event);
            }
        }
    }

    flush_section(current_image_event, current_text_events, is_centered);
    let final_html = final_html_parts.join("\n");

    view! { <div class="markdown-container" inner_html=final_html></div> }
}

fn fix_image_path(path: &str) -> String {
    let path_str = path.to_string();
    if path_str.starts_with("../") {
        format!("/public/{}", &path_str[3..])
    } else if !path_str.starts_with("/") && !path_str.contains("://") {
        format!("/public/{}", path_str)
    } else if path_str.starts_with("/images") {
        format!("/public{}", path_str)
    } else {
        path_str
    }
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
