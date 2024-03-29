"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, emit } from "@tauri-apps/api/event";

interface ArpEntry {
  ip_address: string;
  mac_address: string;
  hostname: string;
}

export default function Devices() {
  const [arpEntries, setArpEntries] = useState<ArpEntry[]>([]);
  const [isIdentifying, setIsIdentifying] = useState<boolean>(false);
  const [message, setMessage] = useState<string>("Nothing");

  useEffect(() => {
    listen_to_hostnames();

    listen("arp_table", (e) => {
      const payload = e.payload as string;
      const newEntries: ArpEntry[] = JSON.parse(payload);
      setArpEntries([...newEntries]);
    });
  }, []);

  function greet() {
    emit("hostname_request", { ip_address: "10.0.0.31" });
  }

  async function listen_to_hostnames() {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    const unlisten = await listen("hostname_response", (e: any) => {
      setMessage(e.payload);
    });
  }

  async function identifyDevices() {
    setIsIdentifying(true);

    arpEntries.forEach(async (entry, index) => {
      if (entry.hostname === "Unknown") {
        try {
          const hostname = (await invoke("get_hostname", {
            ipAddress: entry.ip_address,
          })) as string;

          setArpEntries((currentEntries) =>
            currentEntries.map((e, i) =>
              i === index ? { ...e, hostname } : e,
            ),
          );
        } catch (error) {
          console.error(
            "Error getting hostname for IP:",
            entry.ip_address,
            error,
          );

          setArpEntries((currentEntries) =>
            currentEntries.map((e, i) =>
              i === index ? { ...e, hostname: "Failed" } : e,
            ),
          );
        }
      }
    });
    setIsIdentifying(false);
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div>{message}</div>
      <button
        type="button"
        className="btn btn-primary drawer-button"
        onClick={greet}
      >
        Greet
      </button>
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
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </main>
  );
}
