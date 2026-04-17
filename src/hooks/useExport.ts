// src/hooks/useExport.ts
// Drop-in hook — call exportTns() from your Export button's onClick.

import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

interface ExportResult {
  saved_path: string;
}

interface CalculatorInfo {
  name: string;
  usb_id: string;
}

// ── Export hook ─────────────────────────────────────────────────────────────

export function useExport() {
  const [exporting, setExporting] = useState(false);
  const [lastSavedPath, setLastSavedPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  /**
   * @param content Raw note text
   * @param filename Suggested filename
   * @param isBold Boolean for bold formatting
   * @param textColor RGB string like "rgb(0,0,0)"
   */
  async function exportTns(
    content: string, 
    filename: string, 
    isBold: boolean = false, 
    textColor: string = "rgb(0,0,0)"
  ) {
    setExporting(true);
    setError(null);
    try {
      const result = await invoke<ExportResult>("export_tns", {
        content,
        filename,
        isBold,      // Matches the Rust snake_case parameter 'is_bold'
        textColor,   // Matches the Rust snake_case parameter 'text_color'
      });
      setLastSavedPath(result.saved_path);
      return result.saved_path;
    } catch (err) {
      const msg = typeof err === "string" ? err : String(err);
      setError(msg);
      return null;
    } finally {
      setExporting(false);
    }
  }

  return { exportTns, exporting, lastSavedPath, error };
}

// ── Calculator list hook ─────────────────────────────────────────────────────

export function useCalculators() {
  const [calculators, setCalculators] = useState<CalculatorInfo[]>([]);
  const [scanning, setScanning] = useState(false);

  async function scan() {
    setScanning(true);
    try {
      const devices = await invoke<CalculatorInfo[]>("list_calculators");
      setCalculators(devices);
    } finally {
      setScanning(false);
    }
  }

  async function sendToCalculator(
    filePath: string,
    usbId: string,
    remotePath = "/documents/note.tns"
  ) {
    return invoke<string>("send_to_calculator", {
      filePath,
      usbId,
      remotePath,
    });
  }

  return { calculators, scanning, scan, sendToCalculator };
}
