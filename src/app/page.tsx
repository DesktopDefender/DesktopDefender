"use client";

import DDText from "@/components/core/DDText";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export default function Home() {
  const [connected, setConnected] = useState<boolean>(false);

  useEffect(() => {
    listen("connection_status", (e) => {
      const payload = e.payload as boolean;
      //const status: boolean = JSON.parse(payload);
      setConnected(payload);
    });
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div>
        <DDText className="text-3xl">Want me to protect your network?</DDText>
        {connected ? (
          <DDText className="text-xl text-center">Call me the defender</DDText>
        ) : (
          <DDText className="text-xl text-center">
            What network little bro? 😂
          </DDText>
        )}
      </div>
    </main>
  );
}
