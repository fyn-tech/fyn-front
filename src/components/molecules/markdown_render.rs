use leptos::prelude::*;
use regex::Regex;

use crate::common::size::Size;
use crate::components::atoms::layout::{FlexAlign, Stack};
use crate::components::atoms::typography::{A, H1, H2, H3, H4, H4_CLASS, P};

// -------------------------------------------------------------------------------------------------
// Supporting Parse Helper
// -------------------------------------------------------------------------------------------------

fn parse_markdown(content: &str) -> Vec<AnyView> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Headers
        if line.starts_with("# ") {
            let text = line[2..].trim().to_string();
            result.push(view! { <H1>{text}</H1> }.into_any());
            i += 1;
        } else if line.starts_with("## ") {
            let text = line[3..].trim().to_string();
            result.push(view! { <H2>{text}</H2> }.into_any());
            i += 1;
        } else if line.starts_with("### ") {
            let text = line[4..].trim().to_string();
            result.push(view! { <H3>{text}</H3> }.into_any());
            i += 1;
        } else if line.starts_with("#### ") {
            let text = line[5..].trim().to_string();
            result.push(view! { <H4>{text}</H4> }.into_any());
            i += 1;
        }
        // List items
        else if line.starts_with("- ") {
            let (list_items, consumed) = parse_list(&lines[i..]);
            result.push(
                view! {
                    <Stack size={Size::Xs} align={FlexAlign::Stretch}>
                        {list_items.into_iter().collect::<Vec<_>>()}
                    </Stack>
                }
                .into_any(),
            );
            i += consumed;
        }
        // Regular paragraph
        else {
            let (paragraph_lines, consumed) = collect_paragraph(&lines[i..]);
            let paragraph_content = paragraph_lines.join(" ");
            let parsed_paragraph = parse_inline_elements(&paragraph_content);
            result.push(view! { <P>{parsed_paragraph}</P> }.into_any());
            i += consumed;
        }
    }

    result
}

fn parse_list(lines: &[&str]) -> (Vec<AnyView>, usize) {
    let mut items = Vec::new();
    let mut consumed = 0;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("- ") {
            let item_text = trimmed[2..].trim();
            let parsed_item = parse_inline_elements(item_text);
            items.push(
                view! {
                    <Stack size={Size::None} horizontal=true align={FlexAlign::Start}>
                        <span class="mr-2">"•"</span>
                        <span>{parsed_item}</span>
                    </Stack>
                }
                .into_any(),
            );
            consumed += 1;
        } else if trimmed.is_empty() {
            consumed += 1;
            break;
        } else {
            break;
        }
    }

    (items, consumed)
}

fn collect_paragraph(lines: &[&str]) -> (Vec<String>, usize) {
    let mut paragraph_lines = Vec::new();
    let mut consumed = 0;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#") || trimmed.starts_with("-") {
            break;
        }
        paragraph_lines.push(trimmed.to_string());
        consumed += 1;
    }

    // If we didn't consume any lines, consume at least one to avoid infinite loop
    if consumed == 0 && !lines.is_empty() {
        paragraph_lines.push(lines[0].trim().to_string());
        consumed = 1;
    }

    (paragraph_lines, consumed)
}

fn parse_inline_elements(text: &str) -> Vec<AnyView> {
    // Simple regex for markdown links: [text](url)
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let mut result = Vec::new();
    let mut last_end = 0;

    for cap in link_regex.find_iter(text) {
        // Add text before the link
        if cap.start() > last_end {
            let text_before = &text[last_end..cap.start()];
            if !text_before.is_empty() {
                result.push(view! { <span>{text_before.to_string()}</span> }.into_any());
            }
        }

        // Extract link parts
        let full_match = cap.as_str();
        if let Some(link_cap) = link_regex.captures(full_match) {
            let link_text = link_cap.get(1).unwrap().as_str().to_string();
            let link_url = link_cap.get(2).unwrap().as_str().to_string();

            result.push(
                view! {
                    <A href={link_url} text_class={H4_CLASS.to_string()}>
                        {link_text}
                    </A>
                }
                .into_any(),
            );
        }

        last_end = cap.end();
    }

    // Add remaining text after the last link
    if last_end < text.len() {
        let remaining_text = &text[last_end..];
        if !remaining_text.is_empty() {
            result.push(view! { <span>{remaining_text.to_string()}</span> }.into_any());
        }
    }

    // If no links were found, return the entire text as a span
    if result.is_empty() {
        result.push(view! { <span>{text.to_string()}</span> }.into_any());
    }

    return result;
}

// -------------------------------------------------------------------------------------------------
// Components
// -------------------------------------------------------------------------------------------------

#[component]
pub fn MarkdownRenderer(content: String) -> impl IntoView {
    let parsed_content = parse_markdown(&content);

    view! {
        <Stack size={Size::Lg} align={FlexAlign::Stretch}>
            {parsed_content.into_iter().collect::<Vec<_>>()}
        </Stack>
    }
}
