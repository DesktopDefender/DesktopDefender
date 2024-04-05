import Navbar from "@/components/Navbar";
import type { Metadata } from "next";
import { Outfit } from "next/font/google";
import "./globals.css";

const outfit = Outfit({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${outfit.className} px-24`}>
        <div className="absolute border-DDOrange top-0 z-10 left-0 border-t-2 w-screen" />
        <Navbar />
        {children}
      </body>
    </html>
  );
}
