"use client";

import { RealTimeChart } from "@/components/Chart";
import { PacketTable } from "@/components/PacketTable";

export default function Monitor() {
  return (
    <div className="px-8">
      <RealTimeChart />
      <PacketTable />
    </div>
  );
}
