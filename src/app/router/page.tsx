"use client";

import DDPageContainer from "@/components/DDPageContainer";
import DDText from "@/components/core/DDText";
import ExternalLink from "@/components/core/ExternalLink";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import type { Manufacturer } from "../types/Manufacturer";

export default function Router() {
  const [routerIp, setRouterIp] = useState("");
  const [routerIpLoading, setRouterIpLoading] = useState(false);
  const [routerMac, setRouterMac] = useState("");
  const [routerMacLoading, setRouterMacLoading] = useState(false);
  const [openPorts, setOpenPorts] = useState<number[]>([]);
  const [openPortsLoading, setOpenPortsLoading] = useState(false);
  const [routerVendor, setRouterVendor] = useState("");
  const [routerVendorLoading, setRouterVendorLoading] = useState(false);

  const [infoMessage, setInfoMessage] = useState("");
  const [infoMessageLoading, setInfoMessageLoading] = useState(false);

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
    invoke<number[]>("find_open_ports", { ip: ip, inPorts: [53, 80, 443] })
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

  function getVendorFromMac(mac: string) {
    setRouterVendorLoading(true);
    invoke<string>("get_manufacturer_by_mac", { macAddress: mac })
      .then((vendor) => {
        const v: Manufacturer = JSON.parse(vendor);
        setRouterVendor(`${v.manufacturer} - ${v.country}`);
        console.log("vendor: ", v);
      })
      .catch(console.error)
      .finally(() => setRouterVendorLoading(false));
  }

  useEffect(() => {
    if (routerMac === "") return;
    getVendorFromMac(routerMac);
  }, [routerMac]);

  const renderPorts = (ports: number[]) => {
    return ports.map((p) => (
      <button
        key={p}
        type="button"
        className="px-2 py-1 w-14 text-center rounded-md bg-slate-500 hover:bg-slate-600 active:bg-slate-700"
        onClick={() => {
          setInfoMessageLoading(true);
          invoke<string>("call_http_port", { host: routerIp, port: p })
            .then((res) => {
              const endpoints: string[] = JSON.parse(res);

              console.log("endpoints: ", endpoints);
              setInfoMessage(`${p}: bro is http 😎`);
            })
            .catch((e) => {
              console.error("router error: ", e);
              setInfoMessage(`${p}: bro is not http 🫵😂`);
            })
            .finally(() => setInfoMessageLoading(false));
        }}
      >
        {p}
      </button>
    ));
  };

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
          {openPortsLoading ? (
            <DDText>"Loading..."</DDText>
          ) : (
            renderPorts(openPorts)
          )}
        </div>
        {openPorts.includes(80) ||
          (openPorts.includes(443) && (
            <ExternalLink
              className="bg-slate-600 hover:bg-slate-700 active:bg-slate-800 px-2 py-1 rounded-md"
              url={`http${openPorts.includes(443) ? "s" : ""}://${routerIp}`}
            >
              Admin Portal
            </ExternalLink>
          ))}
        <DDText>{infoMessage}</DDText>
      </div>
    </DDPageContainer>
  );
}
