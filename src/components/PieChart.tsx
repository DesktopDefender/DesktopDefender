import { listen } from "@tauri-apps/api/event";
import type {
  CoreChartOptions,
  DatasetChartOptions,
  ElementChartOptions,
  PluginChartOptions,
} from "chart.js";
import { useEffect, useRef, useState } from "react";
import { Pie } from "react-chartjs-2";

interface IPCache {
  [ipAddress: string]: IpInfo;
}

interface IpInfo {
  flag: string;
  country: string;
  hostname: string;
  ip: string;
}

interface Countries {
  [country: string]: number;
}

interface Dataset {
  label: string;
  data: number[];
  backgroundColor: string[];
  hoverOffset: number;
}

interface ChartData {
  labels: string[];
  datasets: Dataset[];
}

export const CountryPieChart = () => {
  const [countryCount, setCountryCount] = useState<Countries>({});
  const [data, setData] = useState<ChartData>({
    labels: [],
    datasets: [
      {
        label: "Country Overview",
        data: [],
        backgroundColor: [],
        hoverOffset: 3,
      },
    ],
  });

  // Ref for storing color assignments to ensure consistency across re-renders
  const countryColors = useRef<{ [country: string]: string }>({}).current;

  // Predefined array of 20 colors
  const colors = [
    "#FF6633",
    "#FFB399",
    "#FF33FF",
    "#FFFF99",
    "#00B3E6",
    "#E6B333",
    "#3366E6",
    "#999966",
    "#99FF99",
    "#B34D4D",
    "#80B300",
    "#809900",
    "#E6B3B3",
    "#6680B3",
    "#66991A",
    "#FF99E6",
    "#CCFF1A",
    "#FF1A66",
    "#E6331A",
    "#33FFCC",
  ];

  // Function to assign or retrieve the color for a country
  function getColorForCountry(country: string) {
    if (!countryColors[country]) {
      // Assign a new color if one hasn't been assigned yet
      const availableColors = colors.filter(
        (c) => !Object.values(countryColors).includes(c),
      );
      countryColors[country] =
        availableColors.length > 0 ? availableColors[0] : "#000000"; // Fallback to black
    }
    return countryColors[country];
  }

  // I'm sorry Hävard please don't be mad 😭
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const options: any = {
    maintainAspectRatio: false,
    aspectRatio: 4,
    responsive: true,
    plugins: {
      legend: {
        display: true,
        position: "left",
      },
    },
  };

  useEffect(() => {
    const unlisten = listen("info", (e) => {
      const payload = e.payload as string;
      const newIps: IPCache = JSON.parse(payload);
      const counts: Countries = Object.values(newIps).reduce<Countries>(
        (acc, { country }) => {
          acc[country] = (acc[country] || 0) + 1;
          return acc;
        },
        {},
      );

      setCountryCount(counts);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const labels = Object.keys(countryCount);
    const chartData = Object.values(countryCount);
    const backgroundColors = labels.map((country) =>
      getColorForCountry(country),
    );

    setData({
      labels: labels,
      datasets: [
        {
          label: "Country Overview",
          data: chartData,
          backgroundColor: backgroundColors,
          hoverOffset: 4,
        },
      ],
    });
  }, [countryCount]);

  return <Pie data={data} options={options} />;
};
