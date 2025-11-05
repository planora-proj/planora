import type { Metadata } from "next";
import "./globals.css";

import { ThemeProvider } from "@/hooks/theme-provider";
import { ToastProvider } from "@/hooks/toast-provider";

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
        <html lang="en" suppressHydrationWarning>
            <body>
                <ThemeProvider
                    attribute="class"
                    defaultTheme="system"
                    enableSystem
                >
                    {children}
                    <ToastProvider />
                </ThemeProvider>
            </body>
        </html>
    );
}
