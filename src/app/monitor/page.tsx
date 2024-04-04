"use client";

import { RealTimeChart } from "@/components/Chart";
import { IpTable } from "@/components/InfoTable";
import { PacketTable } from "@/components/PacketTable";
import { CountryPieChart } from "@/components/PieChart";

export default function Monitor() {
  return (
    <div className="flex flex-col px-8">
      <div>
        <RealTimeChart />
        <IpTable />
      </div>
      <div className="flex flex-row justify-between items-start">
        <div className="pt-16">
          <CountryPieChart />
        </div>
        <div>
          <PacketTable />
        </div>
      </div>
    </div>
  );
}
