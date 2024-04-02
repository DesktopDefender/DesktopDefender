import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Line } from "react-chartjs-2";
import Chart, {
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js/auto";

Chart.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
);

interface PacketInfo {
  protocol: string;
  source: string;
  destination: string;
  length: number;
}

interface PacketRateInfo {
  count: number;
  timestamp: string;
}

interface Dataset {
  label: string;
  data: number[];
  borderColor: string;
  tension: number;
}

interface ChartData {
  labels: string[];
  datasets: Dataset[];
}

export const RealTimeChart = () => {
  const [packetRate, setPacketRate] = useState<PacketRateInfo[]>([]);
  const [data, setData] = useState<ChartData>({
    labels: [],
    datasets: [
      {
        label: "Packet Traffic",
        data: [],
        borderColor: "rgb(75, 192, 192)",
        tension: 0.2,
      },
    ],
  });

  useEffect(() => {
    const unlisten = listen("packets", (e) => {
      const payload = e.payload as string;
      const newPackets: PacketInfo[] = JSON.parse(payload);
      const timestamp = new Date().toLocaleTimeString(); // Get current time as a string

      setPacketRate((prevRates) => [
        ...prevRates,
        { count: newPackets.length, timestamp: timestamp },
      ]);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      const updatedData: ChartData = {
        labels: packetRate.map((rate) => rate.timestamp),
        datasets: [
          {
            ...data.datasets[0],
            data: packetRate.map((rate) => rate.count),
          },
        ],
      };

      if (updatedData.labels.length > 20) {
        updatedData.labels.shift();
        updatedData.datasets[0].data.shift();
      }
      setData(updatedData);
    }, 1000);

    return () => clearInterval(interval);
  }, [packetRate, data.datasets[0]]);

  return <Line data={data} />;
};
