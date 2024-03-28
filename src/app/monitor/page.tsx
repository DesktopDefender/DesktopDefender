"use client";

import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface PacketInfo {
	protocol: string;
	source: string;
	destination: string;
	length: number;
}

export default function Monitor() {
	const [packets, setPackets] = useState<PacketInfo[]>([]);

	useEffect(() => {
		const interval = setInterval(() => {
			invoke<string>("listen_to_traffic")
				.then((response) => {
					const data: PacketInfo[] = JSON.parse(response);
					console.log(data);
					setPackets((currentPackets) => [...currentPackets, ...data]);
				})
				.catch((error: Error) =>
					console.error("Error invoking listen_to_traffic:", error),
				);
		}, 500);

		return () => clearInterval(interval);
	}, []);

	return (
		<main className="flex min-h-screen flex-col items-center justify-between p-24">
			<ul>
				{packets.map((packet, index) => (
					<li className="text-white" key={index}>
						Protocol: {packet.protocol}, Source: {packet.source}, Destination:{" "}
						{packet.destination}, Length: {packet.length}
					</li>
				))}
			</ul>
		</main>
	);
}
