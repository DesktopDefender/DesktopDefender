"use client";

import { Card } from "@/components/Card";
import DDPageContainer from "@/components/DDPageContainer";
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
    <DDPageContainer>
      <h1 className="self-center text-3xl font-bold mb-4">Desktop defender</h1>
      <div className="grid grid-cols-2 gap-4">
        <Card
          title="Router"
          body="My router"
          link="/router"
          buttonText="Go to Router"
        />
        <Card
          title="Devices"
          body="See which devices are on you network"
          link="/devices"
          buttonText="See Devices"
        />
        <Card
          title="Traffic"
          body="See statistics about your network traffic and usage"
          link="/monitor"
          buttonText="See Traffic"
        />
        <Card
          title="Settings"
          body="Configure Desktop Defender to your liking"
          link="/"
          buttonText="Go to Settings"
        />
      </div>
    </DDPageContainer>
  );
}
