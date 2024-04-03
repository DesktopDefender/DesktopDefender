"use client";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

interface Network {
  mac_address: string;
  ip_address: string;
  manufacturer: string;
  country: string;
}

interface Device {
  mac_address: string;
  ip_address: string;
  hostname: string;
  manufacturer: string;
  country: string;
  date_added: string;
}

export default function Devices() {
  const [network, setNetwork] = useState<Network | undefined>(undefined);
  const [devices, setDevices] = useState<Device[]>([]);

  useEffect(() => {
    invoke("get_router_info")
      .then((response) => {
        const network: Network = JSON.parse(response as string);
        setNetwork(network);

        initalize_devices(network.mac_address);
      })
      .catch((error) =>
        console.error("Error fetching network devices:", error),
      );

    listen("hostname_found", (e) => {
      if (network?.mac_address) {
        get_network_info(network.mac_address);
      }
    });
  }, [network?.mac_address]);

  function initalize_devices(routerMac: string) {
    invoke("initalize_devices", { routerMac: routerMac })
      .then((response) => {
        const devicesArray: Device[] = JSON.parse(response as string);
        setDevices(devicesArray);
        console.log(devicesArray);
      })
      .catch((error) =>
        console.error("Error fetching network devices:", error),
      );

    emit("hostname_request", { router_mac: routerMac });
  }

  function get_network_info(routerMac: string) {
    invoke("get_network_info", { routerMac: routerMac })
      .then((response) => {
        const devicesArray: Device[] = JSON.parse(response as string);
        setDevices(devicesArray);
        console.log(devicesArray);
      })
      .catch((error) =>
        console.error("Error fetching network devices:", error),
      );
  }

  useEffect(() => {
    const interval = setInterval(() => {
      emit("hostname_request", { router_mac: network?.mac_address });
    }, 30000);

    return () => clearInterval(interval);
  }, [network?.mac_address]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-2">
      <div className="text-center">
        <p className="m-10 mb-2 text-3xl">Your router</p>
        <div className="w-96 h-32 rounded-lg bg-DDOrange m-12 mt-0 p-4">
          <p>{network?.manufacturer}</p>
          <p>IP Address: {network?.ip_address}</p>
          <p>MAC Address: {network?.mac_address}</p>
          <p>{network?.country}</p>
        </div>
      </div>

      <div className="overflow-x-auto">
        <table className="table table-zebra">
          <thead>
            <tr>
              <th />
              <th>IP Address</th>
              <th>MAC Address</th>
              <th>Hostname</th>
              <th>Manufacturer</th>
              <th>Country</th>
              <th>Registered</th>
            </tr>
          </thead>
          <tbody>
            {devices.map((entry, index) => (
              <tr>
                <th>{index + 1}</th>
                <td>{entry.ip_address}</td>
                <td>{entry.mac_address}</td>
                <td>{entry.hostname}</td>
                <td>{entry.manufacturer}</td>
                <td>{entry.country}</td>
                <td>{entry.date_added}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </main>
  );
}
