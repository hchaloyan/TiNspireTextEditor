import { useState } from "react";
import type { NoteFile } from "../App";
import "./Sidebar.css";

interface Props {
  files: NoteFile[];
  activeId: string;
  onSelect: (id: string) => void;
  onNew: () => void;
  onDelete: (id: string) => void;
  calcConnected: boolean;
  onToggleCalc: () => void;
}

export default function Sidebar({ files, activeId, onSelect, onNew, onDelete, calcConnected, onToggleCalc }: Props) {
  const [hoverId, setHoverId] = useState<string | null>(null);

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <span className="sidebar-label">Notes</span>
        <button className="new-btn" onClick={onNew} title="New file">+</button>
      </div>

      <div className="file-list">
        {files.map((f) => (
          <div
            key={f.id}
            className={`file-item ${f.id === activeId ? "active" : ""}`}
            onClick={() => onSelect(f.id)}
            onMouseEnter={() => setHoverId(f.id)}
            onMouseLeave={() => setHoverId(null)}
          >
            <span className="file-icon">
              <svg width="11" height="13" viewBox="0 0 11 13" fill="none">
                <path d="M1 1h6l3 3v8H1V1z" stroke="currentColor" strokeWidth="0.8"/>
                <path d="M7 1v3h3" stroke="currentColor" strokeWidth="0.8"/>
              </svg>
            </span>
            <span className="file-name">{f.name}</span>
            {hoverId === f.id && files.length > 1 && (
              <button
                className="delete-btn"
                onClick={(e) => { e.stopPropagation(); onDelete(f.id); }}
                title="Delete"
              >×</button>
            )}
          </div>
        ))}
      </div>

      <div className="calc-panel">
        <div className="calc-status">
          <div className={`status-dot ${calcConnected ? "connected" : ""}`} />
          <span className="calc-name">
            {calcConnected ? "TI-Nspire CX" : "No calculator"}
          </span>
        </div>
        <button className="calc-btn" onClick={onToggleCalc}>
          {calcConnected ? "Disconnect" : "Connect calculator"}
        </button>
      </div>
    </aside>
  );
}
