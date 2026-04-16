import { useState } from "react";

interface ConnectionManagerProps {
  status: string;
}

export function ConnectionManager({ status }: ConnectionManagerProps) {
  // This state will eventually be populated by your calculator detection logic
  const [availableDevices, setAvailableDevices] = useState<string[]>([]);

  return (
    <div className="connection-manager">
      <div className="status-header">
        <span className={`status-dot ${status === "Connected" ? "connected" : ""}`}></span>
        <span className="status-text">{status}</span>
      </div>
      <select 
        className="device-dropdown" 
        disabled={availableDevices.length === 0}
        defaultValue=""
      >
        <option value="" disabled>
          {availableDevices.length > 0 ? "Select a device..." : "No devices detected"}
        </option>
        {availableDevices.map((device) => (
          <option key={device} value={device}>
            {device}
          </option>
        ))}
      </select>
    </div>
  );
}