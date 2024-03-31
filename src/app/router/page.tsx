"use client";

import DDText from "@/components/DDText";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export default function Router() {
  const [routerIp, setRouterIp] = useState("");
  const [routerIpLoading, setRouterIpLoading] = useState(false);

  const getRouterIp = () => {
    if (routerIp !== "") return;
    setRouterIpLoading(true);
    invoke<string>("find_ip")
      .then((result) => {
        console.log(`result found: ${result}`);
        setRouterIp(result);
      })
      .catch(console.error)
      .finally(() => setRouterIpLoading(false));
  };

  useEffect(() => {
    getRouterIp();
  }, [getRouterIp]);

  return (
    <div className="border-x-2 border-dashed flex items-center justify-center h-screen">
      <div>
        <DDText className="text-3xl">Hello, Router</DDText>
        <div className="flex justify-between">
          <DDText className="">router IP:</DDText>
          <DDText>{routerIpLoading ? "Loading..." : routerIp}</DDText>
        </div>
      </div>
    </div>
  );
}
