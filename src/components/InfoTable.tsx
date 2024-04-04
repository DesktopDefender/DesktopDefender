"use client";

import React, { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

interface IPCache {
  [ipAddress: string]: IpInfo;
}

interface IpInfo {
  flag: string;
  country: string;
  hostname: string;
  ip: string;
}

export function IpTable() {
  const [ips, setIps] = useState<IPCache>({});

  useEffect(() => {
    const unlisten = listen("info", (e) => {
      const payload = e.payload as string;
      const newIps: IPCache = JSON.parse(payload);
      setIps(() => newIps);
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
              <th>Domain</th>
              <th>Country</th>
              <th>Flag</th>
              <th>Ip</th>
            </tr>
          </thead>
        </table>
        <div
          className="overflow-y-auto no-scrollbar"
          style={{ maxHeight: "200px" }}
        >
          <table className="table">
            <tbody>
              {Object.values(ips).map((ip) => (
                <tr className="animate-fadeIn">
                  <td>{ip.hostname}</td>
                  <td>{ip.country}</td>
                  <td>{ip.flag}</td>
                  <td>{ip.ip}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
