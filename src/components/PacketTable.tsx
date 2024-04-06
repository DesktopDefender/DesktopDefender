"use client";

import React, { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

interface PacketInfo {
  protocol: string;
  source: string;
  destination: string;
  length: number;
}

export function PacketTable() {
  const [packets, setPackets] = useState<PacketInfo[]>([]);

  useEffect(() => {
    const unlisten = listen("packets", (e) => {
      const payload = e.payload as string;
      const newPackets: PacketInfo[] = JSON.parse(payload);

      newPackets.reverse();
      setPackets((currentPackets) =>
        [...newPackets, ...currentPackets].slice(0, 250),
      );
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="flex flex-col items-center justify-between p-4">
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
        <div
          className="overflow-y-auto no-scrollbar"
          style={{ maxHeight: "200px" }}
        >
          <table className="table">
            <tbody>
              {packets.map((packet) => (
                <tr className="animate-fadeIn">
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
    </div>
  );
}
