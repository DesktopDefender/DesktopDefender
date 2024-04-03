"use client";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";



interface Device {
  macAddress: string;
  ipAddress: string;
  hostname: string;
  manufacturer: string;
  country: string;
}


export default function NewDevices() {

  const [routerMAC, setRouterMAC] = useState<string>("d0:6f:82:85:f8:b3");

  const [devices, setDevices] = useState<Device[]>([]);

  // TODO get actual mac address of router

  useEffect(() => {
    get_network_info();
  }, []);


  function get_network_info() {
    invoke('get_network_info', { routerMac: routerMAC })
      .then((response) => {
        const devicesArray: Device[] = JSON.parse(response as string);
        setDevices(devicesArray);
        console.log(devicesArray);
      })
      .catch((error) => console.error('Error fetching network devices:', error));
  }

  useEffect(() => {
    const interval = setInterval(() => {
      emit("hostname_request", { router_mac: routerMAC });
    }, 30000);

    return () => clearInterval(interval);
  }, [routerMAC]); 
  

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="w-96 h-32 border-DDOrange border-2 rounded-lg">
        <p>Your router</p>
        <p>{routerMAC}</p>
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
            </tr>
          </thead>
          <tbody>
            {devices.map((entry, index) => (
              <tr>
                <th>{index + 1}</th>
                <td>{entry.ipAddress}</td>
                <td>{entry.macAddress}</td>
                <td>{entry.hostname}</td>
                <td>{entry.manufacturer}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </main>
  );
}
