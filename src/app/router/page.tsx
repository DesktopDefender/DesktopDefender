"use client";

import DDPageContainer from "@/components/DDPageContainer";
import DDText from "@/components/DDText";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export default function Router() {
  const [routerIp, setRouterIp] = useState("");
  const [routerIpLoading, setRouterIpLoading] = useState(false);
  const [routerMac, setRouterMac] = useState("");
  const [routerMacLoading, setRouterMacLoading] = useState(false);
  const [openPorts, setOpenPorts] = useState<number[]>([]);
  const [openPortsLoading, setOpenPortsLoading] = useState(false);
  const [routerVendor, setRouterVendor] = useState("");
  const [routerVendorLoading, setRouterVendorLoading] = useState(false);

  // function declared using "function"
  function getRouterIp() {
    if (routerIp !== "") return;
    setRouterIpLoading(true);
    invoke<string>("find_ip")
      .then((result) => {
        console.log(`result found: ${result}`);
        setRouterIp(result);
      })
      .catch(console.error)
      .finally(() => setRouterIpLoading(false));
  }

  useEffect(() => {
    getRouterIp();
  }, []);

  function getMacAddressFromIp(ip: string) {
    setRouterMacLoading(true);
    invoke<string>("find_mac_address", { ip: ip })
      .then((mac) => {
        setRouterMac(mac);
        console.log("mac: ", mac);
      })
      .catch(console.error)
      .finally(() => setRouterMacLoading(false));
  }

  useEffect(() => {
    if (routerIp === "") return;
    getMacAddressFromIp(routerIp);
  }, [routerIp]);

  function getOpenPortsFromIp(ip: string) {
    setOpenPortsLoading(true);
    invoke<number[]>("find_open_ports", { ip: ip, inPorts: [] })
      .then((ports) => {
        console.log("ports: ", ports);
        setOpenPorts(ports);
      })
      .catch(console.error)
      .finally(() => setOpenPortsLoading(false));
  }

  useEffect(() => {
    if (routerIp === "") return;
    getOpenPortsFromIp(routerIp);
  }, [routerIp]);

  // const getVendorFromMac = (mac: string) => {
  //   setRouterVendorLoading(true);
  //   invoke<string>("find_vendor", { mac: mac })
  //     .then((vendor) => {
  //       setRouterVendor(vendor);
  //       console.log("vendor: ", vendor);
  //     })
  //     .catch(console.error)
  //     .finally(() => setRouterVendorLoading(false));
  // };

  // useEffect(() => {
  //   if (routerMac === "") return;
  //   getVendorFromMac(routerMac);
  // }, [routerMac, getVendorFromMac]);

  const getVendorFromMac = (mac: string) => {
    setRouterVendorLoading(true);
    invoke<string>("find_vendor", { mac: mac })
      .then((vendor) => {
        setRouterVendor(vendor);
        console.log("vendor: ", vendor);
      })
      .catch(console.error)
      .finally(() => setRouterVendorLoading(false));
  };

  useEffect(() => {
    if (routerMac === "") return;
    getVendorFromMac(routerMac);
  }, [routerMac, getVendorFromMac]);

  return (
    <DDPageContainer>
      <div className="flex-grow">
        <DDText className="text-3xl mx-10 text-center">My Router</DDText>
        <div className="flex justify-between">
          <DDText className="">IP:</DDText>
          <DDText>{routerIpLoading ? "Loading..." : routerIp}</DDText>
        </div>
        <div className="flex justify-between">
          <DDText className="">MAC:</DDText>
          <DDText>{routerMacLoading ? "Loading..." : routerMac}</DDText>
        </div>
        <div className="flex justify-between">
          <DDText className="">Vendor:</DDText>
          <DDText>{routerVendorLoading ? "Loading..." : routerVendor}</DDText>
        </div>
        <div className="flex justify-between">
          <DDText className="">Open ports:</DDText>
          <DDText>
            {openPortsLoading ? "Loading..." : `${openPorts.join(" - ")}`}
          </DDText>
        </div>
      </div>
    </DDPageContainer>
  );
}
