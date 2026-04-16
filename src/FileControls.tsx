import { useState } from "react";

interface FileControlsProps {
  onSendMessage: (fileName: string) => void;
  connectionStatus: string;
}

export function FileControls({ onSendMessage, connectionStatus }: FileControlsProps) {
  const [newFileName, setNewFileName] = useState("");

  const handleCreate = () => {
    if (newFileName.trim()) {
      onSendMessage(newFileName);
      setNewFileName("");
    }
  };

  return (
    <div className="sidebar-footer">
      <div className="new-file-section">
        <input
          type="text"
          className="new-file-input"
          placeholder="Filename..."
          value={newFileName}
          onChange={(e) => setNewFileName(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleCreate()}
        />
        <button className="new-file-btn" onClick={handleCreate}>
          + New File
        </button>
      </div>
      <div className="connection-status">
        <span className={`status-dot ${connectionStatus === "Connected" ? "connected" : ""}`}></span>
        {connectionStatus}
      </div>
    </div>
  );
}