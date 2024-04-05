"use client";

import { RealTimeChart } from "@/components/Chart";
import { IpTable } from "@/components/InfoTable";
import { PacketTable } from "@/components/PacketTable";
import { CountryPieChart } from "@/components/PieChart";
import DDPageContainer from "@/components/DDPageContainer";

export default function Monitor() {
  return (
    <DDPageContainer>
      <>
        <RealTimeChart />
        <IpTable />
        <div className="flex justify-center items-center">
          <div className="pt-8">
            <CountryPieChart />
          </div>
          <PacketTable />
        </div>
      </>
    </DDPageContainer>
  );
}
