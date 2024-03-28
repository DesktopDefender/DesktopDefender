'use client'

import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';


interface ArpEntry {
  ip_address: string;
  mac_address: string;
}

export default function Devices() {

  const [arpEntries, setArpEntries] = useState<ArpEntry[]>([]);

  useEffect(() => {
    invoke<ArpEntry[]>('my_custom_command')
      .then((entries) => {
        setArpEntries(entries);
      })
      .catch((error) => {
        console.error('Failed to load ARP entries:', error);
      });
  }, []);


  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {arpEntries.map((entry, index) => (
        <div key={index}>
          IP Address: {entry.ip_address}, MAC Address: {entry.mac_address}
        </div>
      ))}
    </main>
  );
}
