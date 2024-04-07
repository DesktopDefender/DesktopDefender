"use client";

import { ConnectionStatus } from "@/components/ConnectionStatus";
import { Card } from "@/components/Card";
import DDPageContainer from "@/components/DDPageContainer";

export default function Home() {
  return (
    <DDPageContainer>
      <ConnectionStatus />
      <h1 className="self-center text-3xl font-bold mb-4">Desktop defender</h1>
      <div className="grid grid-cols-2 gap-4">
        <Card title="Router" body="My router" link="/router" />
        <Card
          title="Devices"
          body="See which devices are on you network"
          link="/devices"
        />
        <Card
          title="Traffic"
          body="See statistics about your network traffic and usage"
          link="/monitor"
        />
        <Card
          title="Settings"
          body="Configure Desktop Defender to your liking"
          link="/"
        />
      </div>
    </DDPageContainer>
  );
}
