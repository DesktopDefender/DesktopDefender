"use client";

import DDPageContainer from "@/components/DDPageContainer";
import DDText from "@/components/core/DDText";
import ExternalLink from "@/components/core/ExternalLink";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import type { Manufacturer } from "../types/Manufacturer";

enum AdminCredentials {
  FOUND = "FOUND",
  NOT_FOUND = "NOT_FOUND",
  ERROR = "ERROR",
  UNKNOWN = "UNKNOWN",
  LOADING = "LOADING",
}

export default function Router() {
  const [routerIp, setRouterIp] = useState("");
  const [routerIpLoading, setRouterIpLoading] = useState(false);
  const [routerMac, setRouterMac] = useState("");
  const [routerMacLoading, setRouterMacLoading] = useState(false);
  const [openPorts, setOpenPorts] = useState<number[]>([]);
  const [openPortsLoading, setOpenPortsLoading] = useState(false);
  const [routerVendor, setRouterVendor] = useState("");
  const [routerVendorLoading, setRouterVendorLoading] = useState(false);
  const [adminCreds, setAdminCreds] = useState<AdminCredentials>(
    AdminCredentials.UNKNOWN,
  );

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

  const checkCredentials = (port: number) => {
    setAdminCreds(AdminCredentials.LOADING);

    invoke<string | null>("check_admin_creds", { host: routerIp, port: port })
      .then((res) => {
        console.log("creds: ", res);

        if (!res) {
          setAdminCreds(AdminCredentials.NOT_FOUND);
        } else {
          setAdminCreds(AdminCredentials.FOUND);
        }
      })
      .catch((e) => {
        console.error("router error: ", e);
        setAdminCreds(AdminCredentials.ERROR);
      });
  };

  return (
    <DDPageContainer>
      <div className="self-center card w-full bg-base-200 shadow-xl my-4">
        <div className="card-body">
          <h2 className="card-title">Router Information</h2>
          <p>
            Router Vendor: {routerVendorLoading ? "Loading..." : routerVendor}
          </p>
          <p>IP Address: {routerIpLoading ? "Loading..." : routerIp}</p>
          <p>Mac Address: {routerMacLoading ? "Loading..." : routerMac}</p>
          {openPortsLoading ? (
            <DDText>"Loading..."</DDText>
          ) : (
            <DDText>{`${openPorts.length} ports found`}</DDText>
          )}
        </div>
      </div>
      <div className="flex-grow">
        {(openPorts.includes(80) || openPorts.includes(443)) && (
          <div className="flex justify-center gap-10 items-center mt-24 mb-12">
            <ExternalLink
              className="btn btn-outline btn-primary btn-lg"
              url={`http${openPorts.includes(443) ? "s" : ""}://${routerIp}`}
            >
              Admin Portal
            </ExternalLink>
            <div>
              <button
                type="button"
                onClick={() =>
                  checkCredentials(openPorts.includes(443) ? 443 : 80)
                }
                className="btn btn-outline btn-primary btn-lg"
              >
                Admin Credentials
              </button>
            </div>
          </div>
        )}
        <div className="flex justify-center items-center h-full">
          <div
            className={`text-center font-bold p-4 rounded ${
              adminCreds === AdminCredentials.LOADING
                ? "loading loading-spinner loading-md text-neutral"
                : adminCreds === AdminCredentials.ERROR
                  ? "alert alert-warning text-white"
                  : adminCreds === AdminCredentials.NOT_FOUND
                    ? "alert alert-info text-white"
                    : adminCreds === AdminCredentials.FOUND
                      ? "alert alert-error text-white"
                      : "alert alert-info text-white"
            }`}
          >
            {adminCreds === AdminCredentials.LOADING
              ? "Loading..."
              : adminCreds === AdminCredentials.ERROR
                ? "Error checking creds, Router Type not supported yet"
                : adminCreds === AdminCredentials.NOT_FOUND
                  ? "No credentials found"
                  : adminCreds === AdminCredentials.FOUND
                    ? "CREDENTIALS FOR ROUTER FOUND, Please update your password"
                    : "Admin Credentials not checked"}
          </div>
        </div>
      </div>
    </DDPageContainer>
  );
}
