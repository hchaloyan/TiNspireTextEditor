import { useState } from "react";
import Sidebar from "./components/Sidebar";
import Toolbar from "./components/Toolbar";
import Editor from "./components/Editor";
import Preview from "./components/Preview";
import "./App.css";

export interface NoteFile {
  id: string;
  name: string;
  content: string;
}

const defaultFiles: NoteFile[] = [
  { id: "1", name: "physics_notes", content: "# Physics Notes\n\nNewton's **second law** states that `F = ma`.\n\nThe acceleration of an object depends on the net force and its mass.\n\n## Key Formulas\n\n- F = ma\n- p = mv\n- KE = ½mv²" },
  { id: "2", name: "calculus_ch3", content: "# Calculus Chapter 3\n\nDerivatives and their applications.\n\n## The Chain Rule\n\nIf `y = f(g(x))`, then `dy/dx = f'(g(x)) · g'(x)`." },
  { id: "3", name: "chemistry_lab", content: "# Chemistry Lab Notes\n\nExperiment: Titration of acetic acid.\n\n## Observations\n\nThe solution turned pink at **23.4 mL** of NaOH." },
];

export type ViewMode = "edit" | "preview" | "split";

export default function App() {
  const [files, setFiles] = useState<NoteFile[]>(defaultFiles);
  const [activeId, setActiveId] = useState("1");
  const [viewMode, setViewMode] = useState<ViewMode>("split");
  const [calcConnected, setCalcConnected] = useState(false);

  const activeFile = files.find((f) => f.id === activeId)!;

  const updateContent = (content: string) => {
    setFiles((prev) =>
      prev.map((f) => (f.id === activeId ? { ...f, content } : f))
    );
  };

  const renameFile = (name: string) => {
    setFiles((prev) =>
      prev.map((f) => (f.id === activeId ? { ...f, name } : f))
    );
  };

  const newFile = () => {
    const id = Date.now().toString();
    const file: NoteFile = { id, name: "untitled", content: "" };
    setFiles((prev) => [...prev, file]);
    setActiveId(id);
  };

  const deleteFile = (id: string) => {
    setFiles((prev) => prev.filter((f) => f.id !== id));
    if (activeId === id) setActiveId(files[0]?.id ?? "");
  };

  return (
    <div className="app">
      <Sidebar
        files={files}
        activeId={activeId}
        onSelect={setActiveId}
        onNew={newFile}
        onDelete={deleteFile}
        calcConnected={calcConnected}
        onToggleCalc={() => setCalcConnected((v) => !v)}
      />
      <div className="main">
        <Toolbar
          fileName={activeFile.name}
          onRename={renameFile}
          viewMode={viewMode}
          onViewMode={setViewMode}
          content={activeFile.content}
          fileName2={activeFile.name}
        />
        <div className="editor-area">
          {(viewMode === "edit" || viewMode === "split") && (
            <Editor
              content={activeFile.content}
              onChange={updateContent}
              split={viewMode === "split"}
            />
          )}
          {(viewMode === "preview" || viewMode === "split") && (
            <Preview
              content={activeFile.content}
              split={viewMode === "split"}
            />
          )}
        </div>
      </div>
    </div>
  );
}
