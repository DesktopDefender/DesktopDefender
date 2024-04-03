"use client";

import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, emit } from "@tauri-apps/api/event";
import DDText from "@/components/DDText";

interface ArpEntry {
  ip_address: string;
  mac_address: string;
  hostname: string;
  manufacturer: string;
}

export default function Devices() {
  const [arpEntries, setArpEntries] = useState<ArpEntry[]>([]);
  const [isIdentifying, setIsIdentifying] = useState<boolean>(false);

  useEffect(() => {
    listen_to_hostnames();

    listen("arp_table", (e) => {
      const payload = e.payload as string;
      const newEntries: ArpEntry[] = JSON.parse(payload);
      console.log(newEntries);
      setArpEntries([...newEntries]);
    });
  }, []);

  async function listen_to_hostnames() {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    const unlisten = await listen("hostname_response", (e: any) => {
      console.log("recieved something");

      const response = e.payload;

      setArpEntries((currentEntries) =>
        currentEntries.map((entry) =>
          entry.ip_address === response.ip_address
            ? { ...entry, hostname: response.hostname }
            : entry,
        ),
      );
    });
  }

  function identifyDevices() {
    console.log("sending something");
    for (const entry of arpEntries) {
      if (entry.hostname === "Unknown") {
        emit("hostname_request", { ip_address: entry.ip_address });
        console.log("Hostname request emitted for:", entry.ip_address);
      }
    }
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <button
        type="button"
        className="btn btn-primary drawer-button"
        onClick={identifyDevices}
      >
        Identify
      </button>
      <div className="overflow-x-auto">
        <table className="table table-zebra">
          <thead>
            <tr>
              <th />
              <th>IP Address</th>
              <th>MAC Address</th>
              <th>Hostname</th>
              <th>Manufacturer</th>
            </tr>
          </thead>
          <tbody>
            {arpEntries.map((entry, index) => (
              <tr>
                <th>{index + 1}</th>
                <td>{entry.ip_address}</td>
                <td>{entry.mac_address}</td>
                {isIdentifying ? (
                  <td>
                    <span className="loading loading-spinner loading-md" />
                  </td>
                ) : (
                  <td>{entry.hostname}</td>
                )}
                <td>{entry.manufacturer}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </main>
  );
}
