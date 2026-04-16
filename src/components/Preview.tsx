import { useMemo } from "react";
import "./Preview.css";

interface Props {
  content: string;
  split: boolean;
}

function parseMarkdown(md: string): string {
  let html = md
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");

  // headings
  html = html.replace(/^### (.+)$/gm, "<h3>$1</h3>");
  html = html.replace(/^## (.+)$/gm, "<h2>$1</h2>");
  html = html.replace(/^# (.+)$/gm, "<h1>$1</h1>");

  // bold, italic, code
  html = html.replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>");
  html = html.replace(/\*(.+?)\*/g, "<em>$1</em>");
  html = html.replace(/`(.+?)`/g, "<code>$1</code>");

  // unordered lists
  html = html.replace(/^- (.+)$/gm, "<li>$1</li>");
  html = html.replace(/(<li>.*<\/li>\n?)+/gs, (match) => `<ul>${match}</ul>`);

  // paragraphs: lines not already wrapped
  const lines = html.split("\n");
  const result: string[] = [];
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) { result.push(""); continue; }
    if (/^<(h[123]|ul|li|\/ul)/.test(trimmed)) { result.push(trimmed); continue; }
    result.push(`<p>${trimmed}</p>`);
  }

  return result.join("\n");
}

export default function Preview({ content, split }: Props) {
  const html = useMemo(() => parseMarkdown(content), [content]);

  return (
    <div className={`preview-pane ${split ? "split" : "full"}`}>
      <div className="pane-label">preview</div>
      <div
        className="preview-body"
        dangerouslySetInnerHTML={{ __html: html }}
      />
    </div>
  );
}
