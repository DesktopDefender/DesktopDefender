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
		<main className="flex min-h-screen flex-col items-center justify-between p-4">
			<div className="overflow-x-auto">
				<table className="table">
					<thead>
						<tr>
							<th></th>
							<th>Protocol</th>
							<th>Source</th>
							<th>Destination</th>
							<th>Length</th>
						</tr>
					</thead>
				</table>
				<div className="overflow-y-auto" style={{ maxHeight: "200px" }}>
					<table className="table">
						<tbody>
							{packets.map((packet, index) => (
								<tr className="bg-base-200">
									<th>{index}</th>
									<td>{packet.protocol}</td>
									<td>{packet.source}</td>
									<td>{packet.destination}</td>
									<td>{packet.length}</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>
			</div>
		</main>
	);
}
