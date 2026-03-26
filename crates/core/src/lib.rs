use comrak::{markdown_to_html, Options};
use ipc_contracts::{DocumentStats, RenderMarkdownResponse};

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn render_markdown(markdown: &str) -> RenderMarkdownResponse {
    let html = markdown_to_html(markdown, &Options::default());

    let stats = DocumentStats {
        words: count_words(markdown),
        characters: markdown.chars().count(),
        lines: markdown.lines().count().max(1),
    };

    RenderMarkdownResponse { html, stats }
}
