"use client";

import React, { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { RealTimeChart } from "@/components/Chart";

interface PacketInfo {
  protocol: string;
  source: string;
  destination: string;
  length: number;
}

export default function Monitor() {
  const [packets, setPackets] = useState<PacketInfo[]>([]);

  useEffect(() => {
    listen("packets", (e) => {
      const payload = e.payload as string;
      const newPackets: PacketInfo[] = JSON.parse(payload);
      setPackets((currentPackets) => [...currentPackets, ...newPackets]);
    });
  }, []);

  return (
    <div>
      <RealTimeChart />
      <main className="flex min-h-screen flex-col items-center justify-between p-4">
        <div className="overflow-x-auto">
          <table className="table">
            <thead>
              <tr>
                <th>Protocol</th>
                <th>Source</th>
                <th>Destination</th>
                <th>Length</th>
              </tr>
            </thead>
          </table>
          <div className="overflow-y-auto" style={{ maxHeight: "200px" }}>
            <table className="table">
              <tbody>
                {packets.map((packet, index) => (
                  <tr className="bg-base-200">
                    <th>{index}</th>
                    <td>{packet.protocol}</td>
                    <td>{packet.source}</td>
                    <td>{packet.destination}</td>
                    <td>{packet.length}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </main>
    </div>
  );
}
