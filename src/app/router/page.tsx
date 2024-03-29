"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export default function Router() {
  const [greet, setGreet] = useState("");

  const greetFromRust = () => {
    invoke<string>("greet", { name: "Rust" })
      .then((result) => {
        console.log("rust called!");
        setGreet(result);
      })
      .catch(console.error);
  };

  return (
    <div className="min-h-screen">
      <p>{`hello, ${greet}`}</p>
      <div className=" flex justify-center h-96">
        <div className="flex-col flex justify-center">
          <div>
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
      </div>
    </div>
  );
}
