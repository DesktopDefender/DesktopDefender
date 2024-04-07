"use client";

import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export const ConnectionStatus = () => {
  const [connected, setConnected] = useState<boolean>(false);

  useEffect(() => {
    listen("connection_status", (e) => {
      const payload = e.payload as boolean;
      setConnected(payload);
    });
  }, []);

  return (
    <div>
      {connected ? (
        <div className="flex gap-1 items-center">
          <p className="pr-1">Connected</p>
          <div className="w-2.5 h-2.5 rounded-full bg-green-500" />
          <div className="w-2.5 h-2.5 rounded-full bg-green-500" />
          <div className="w-2.5 h-2.5 rounded-full bg-green-500" />
        </div>
      ) : (
        <div className="flex gap-1 items-center">
          <p className="pr-1">No Connection</p>
          <div className="w-2.5 h-2.5 rounded-full bg-red-500" />
          <div className="w-2.5 h-2.5 rounded-full bg-red-500" />
          <div className="w-2.5 h-2.5 rounded-full bg-red-500" />
        </div>
      )}
    </div>
  );
};
