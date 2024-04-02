"use client";

import { RealTimeChart } from "@/components/Chart";
import { IpTable } from "@/components/InfoTable";
import { PacketTable } from "@/components/PacketTable";

export default function Monitor() {
  return (
    <div className="px-8">
      <RealTimeChart />
      <IpTable />
      <PacketTable />
    </div>
  );
}
