"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export default function Router() {
  const [greet, setGreet] = useState("");

  const greetFromRust = () => {
    invoke<string>("greet", { name: "Rust" })
      .then((result) => {
        setGreet(result);
      })
      .catch(console.error);
  };

  return (
    <div className="border-2 flex items-center justify-center h-max">
      <div>
        <p>{`hello, ${greet}`}</p>
        <button
          type="button"
          className="bg-slate-600 shadow-md shadow-slate-600 p-2 hover:shadow-slate-700 active:shadow-none m-4 rounded-md w-40 hover:bg-slate-700 active:bg-slate-800 text-lg h-20"
          onClick={greetFromRust}
        >
          Greet from rust
        </button>
        <button
          type="button"
          className="bg-slate-600 shadow-md shadow-slate-600 p-2 hover:shadow-slate-700 active:shadow-none m-4 rounded-md w-40 hover:bg-slate-700 active:bg-slate-800 text-lg h-20"
          onClick={() => setGreet("from TS")}
        >
          Greet from TS
        </button>
      </div>
    </div>
  );
}
