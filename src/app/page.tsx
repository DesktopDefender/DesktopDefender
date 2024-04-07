"use client";

import { ConnectionStatus } from "@/components/ConnectionStatus";

export default function Home() {

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <ConnectionStatus />
    </main>
  );
}
