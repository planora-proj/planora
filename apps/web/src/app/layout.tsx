import type { Metadata } from "next";
import "./globals.css";

import { ThemeProvider } from "@/context/theme-provider";
import { ToastProvider } from "@/context/toast-provider";
import { UserProvider } from "@/context/user-context";
import { fetchUser } from "@/lib/api/auth";

export const metadata: Metadata = {
    title: "Planora Hall",
    description: "Planora Hall - Collaboration Tool",
};

export default async function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    const user = await fetchUser();

    return (
        <html lang="en" suppressHydrationWarning>
            <body>
                <ThemeProvider
                    attribute="class"
                    defaultTheme="system"
                    enableSystem
                >
                    <UserProvider userIn={user}>{children}</UserProvider>
                    <ToastProvider />
                </ThemeProvider>
            </body>
        </html>
    );
}
