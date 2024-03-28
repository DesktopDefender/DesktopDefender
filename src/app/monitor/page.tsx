"use client";

import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Monitor() {
	const [packageInfo, setPackageInfo] = useState("");

	useEffect(() => {
		invoke("listen_to_traffic")
			.then((response: any) => setPackageInfo(response))
			.catch((error: any) =>
				console.error("Error invoking listen_to_traffic:", error),
			);
	}, []); // Empty dependency array means this effect runs once on mount

	return (
		<main className="flex min-h-screen flex-col items-center justify-between p-24">
			<div>{packageInfo}</div>
		</main>
	);
}
