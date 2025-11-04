import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
    title: "Planora Hall",
    description: "Planora Hall - Collaboration Tool",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    );
}
