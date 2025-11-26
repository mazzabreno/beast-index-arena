import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css"; // <--- ESSA LINHA É A MÁGICA

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Beast Index Arena",
  description: "Autonomous On-chain Battles on Solana",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}