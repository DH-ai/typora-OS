use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderMarkdownRequest {
    pub markdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStats {
    pub words: usize,
    pub characters: usize,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderMarkdownResponse {
    pub html: String,
    pub stats: DocumentStats,
}
