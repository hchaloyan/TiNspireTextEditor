import { useEffect, useState } from "react";
import { useExport } from "../hooks/useExport";
import "./ExportButton.css";

interface Props {
  content: string;
  filename?: string;
}

interface Toast {
  message: string;
  type: "success" | "error";
}

export default function ExportButton({ content, filename = "untitled" }: Props) {
  const { exportTns, exporting } = useExport();
  const [toast, setToast] = useState<Toast | null>(null);
  const [visible, setVisible] = useState(false);

  const showToast = (message: string, type: "success" | "error") => {
    setToast({ message, type });
    setVisible(true);
    setTimeout(() => setVisible(false), 2500);
    setTimeout(() => setToast(null), 3000); // clear after fade
  };

  const handleClick = async () => {
    const path = await exportTns(content, filename);
    if (path) {
      showToast(`✓ ${path.split(/[\\/]/).pop()}`, "success");
    } else {
      showToast("✗ Export failed", "error");
    }
  };

  return (
    <>
      <button
        className="export-btn"
        onClick={handleClick}
        disabled={exporting || !content.trim()}
        style={{
          opacity: exporting || !content.trim() ? 0.5 : 1,
          cursor: exporting ? "wait" : "pointer",
        }}
      >
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" style={{ marginRight: 6 }}>
          <path d="M6 1v7M3 5l3 3 3-3M1 10h10" stroke="currentColor" strokeWidth="1.2" strokeLinecap="round"/>
        </svg>
        {exporting ? "Exporting…" : "Export .tns"}
      </button>

      {toast && (
        <div className={`export-toast ${toast.type} ${visible ? "toast-in" : "toast-out"}`}>
          {toast.message}
        </div>
      )}
    </>
  );
}