// src/components/ExportButton.tsx
// Wire this wherever your toolbar's "Export .tns" button lives.
//
// Usage:
//   <ExportButton content={editorText} filename={noteTitle} />

import React from "react";
import { useExport } from "../hooks/useExport";

interface Props {
  /** Raw text from the editor */
  content: string;
  /** Suggested filename stem, e.g. "my-note" */
  filename?: string;
}

export default function ExportButton({ content, filename = "untitled" }: Props) {
  const { exportTns, exporting, lastSavedPath, error } = useExport();

  async function handleClick() {
    const path = await exportTns(content, filename);
    if (path) {
      console.log("Saved to:", path);
    }
  }

  return (
    <div style={{ display: "inline-flex", flexDirection: "column", gap: 4 }}>
      <button
        onClick={handleClick}
        disabled={exporting || !content.trim()}
        style={{
          padding: "6px 16px",
          cursor: exporting ? "wait" : "pointer",
          opacity: !content.trim() ? 0.5 : 1,
        }}
      >
        {exporting ? "Exporting…" : "Export .tns"}
      </button>

      {lastSavedPath && !error && (
        <span style={{ fontSize: 12, color: "green" }}>
          ✓ Saved: {lastSavedPath}
        </span>
      )}

      {error && (
        <span style={{ fontSize: 12, color: "red" }}>
          ✗ {error}
        </span>
      )}
    </div>
  );
}
