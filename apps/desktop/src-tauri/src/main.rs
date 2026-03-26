use ipc_contracts::{RenderMarkdownRequest, RenderMarkdownResponse};

#[tauri::command]
fn render_markdown(payload: RenderMarkdownRequest) -> RenderMarkdownResponse {
    typoraos_core::render_markdown(&payload.markdown)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![render_markdown])
        .run(tauri::generate_context!())
        .expect("error while running TyporaOS desktop app");
}
