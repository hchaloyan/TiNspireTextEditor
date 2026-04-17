import { useRef } from "react";
import type { ViewMode } from "../App";
import ExportButton from "./ExportButton";
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

      <ExportButton content={content} filename={fileName2} />
    </div>
  );
}