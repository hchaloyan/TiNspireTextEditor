import { useRef } from "react";
import type { ViewMode } from "../App";
import "./Toolbar.css";

interface Props {
  fileName: string;
  onRename: (name: string) => void;
  viewMode: ViewMode;
  onViewMode: (m: ViewMode) => void;
  content: string;
  fileName2: string;
}

export default function Toolbar({ fileName, onRename, viewMode, onViewMode, content, fileName2 }: Props) {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleExport = async () => {
    // Placeholder: will wire to Tauri backend later
    alert(`Exporting "${fileName2}.tns" — Tauri backend coming soon!`);
  };

  return (
    <div className="toolbar">
      <div className="filename-wrap">
        <span className="filename-prefix">~/notes/</span>
        <input
          ref={inputRef}
          className="filename-input"
          value={fileName}
          onChange={(e) => onRename(e.target.value)}
          spellCheck={false}
        />
        <span className="filename-ext">.tns</span>
      </div>

      <div className="view-tabs">
        {(["edit", "split", "preview"] as ViewMode[]).map((m) => (
          <button
            key={m}
            className={`view-tab ${viewMode === m ? "active" : ""}`}
            onClick={() => onViewMode(m)}
          >
            {m}
          </button>
        ))}
      </div>

      <button className="export-btn" onClick={handleExport}>
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" style={{marginRight: 6}}>
          <path d="M6 1v7M3 5l3 3 3-3M1 10h10" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round"/>
        </svg>
        Export .tns
      </button>
    </div>
  );
}
