const editor = document.querySelector("#editor");
const preview = document.querySelector("#preview");
const statsEl = document.querySelector("#stats");

async function renderMarkdown() {
  const markdown = editor.value;

  if (!window.__TAURI__?.core?.invoke) {
    // Browser fallback for quick static preview if Tauri is not running.
    preview.textContent = markdown;
    const words = markdown.trim() ? markdown.trim().split(/\s+/).length : 0;
    statsEl.textContent = `${words} words • ${markdown.length} chars • ${Math.max(markdown.split("\n").length, 1)} lines`;
    return;
  }

  const response = await window.__TAURI__.core.invoke("render_markdown", {
    payload: { markdown },
  });

  preview.innerHTML = response.html;
  statsEl.textContent = `${response.stats.words} words • ${response.stats.characters} chars • ${response.stats.lines} lines`;
}

editor.addEventListener("input", () => {
  void renderMarkdown();
});

void renderMarkdown();
