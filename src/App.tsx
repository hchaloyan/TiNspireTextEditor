import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FileControls } from "./FileControls";
import { ConnectionManager } from "./ConnectionManager";
import "./App.css";

interface FileEntry {
  name: string;
  path: string;
  is_directory: boolean;
}

function App() {
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [editorContent, setEditorContent] = useState("");
  const [connectionStatus, setConnectionStatus] = useState("Disconnected");
  const [currentPath, setCurrentPath] = useState("/");

  useEffect(() => {
    checkConnection();
    loadFiles(currentPath);
  }, []);

  async function checkConnection() {
    try {
      const status = await invoke<string>("check_connection");
      setConnectionStatus(status);
    } catch {
      setConnectionStatus("Disconnected");
    }
  }

  async function loadFiles(path: string) {
    try {
      const entries: FileEntry[] = await invoke("list_files", { path });
      setFiles(entries);
      setCurrentPath(path);
    } catch (e) {
      console.error("Failed to load files:", e);
    }
  }

  async function openFile(filePath: string) {
    try {
      const content: string = await invoke("read_file", { path: filePath });
      setEditorContent(content);
    } catch (e) {
      console.error("Failed to open file:", e);
    }
  }

  async function createNewFile(fileName: string) {
    try {
      await invoke("create_file", { path: currentPath + "/" + fileName });
      loadFiles(currentPath);
    } catch (e) {
      console.error("Failed to create file:", e);
    }
  }

  function navigateUp() {
    if (currentPath === "/") return;
    const parentPath = currentPath.split("/").slice(0, -1).join("/") || "/";
    loadFiles(parentPath);
  }

  return (
    <div className="app-container">
      <aside className="sidebar">
        <div className="sidebar-header">
          <h3>Calculator Files</h3>
        </div>

        {/* Moved higher up and upgraded to Manager */}
        <ConnectionManager status={connectionStatus} />

        <div className="file-list">
          <button className="back-btn" onClick={navigateUp} disabled={currentPath === "/"}>
            ^
          </button>
          <div className="current-path">{currentPath}</div>
          {/* ... file mapping */}
        </div>
        
        <FileControls onSendMessage={createNewFile} />
      </aside>

      <main className="editor-container">
        {/* ... editor */}
      </main>
    </div>
  );
}

export default App;