"use client";

import { Toaster } from "@/components/ui/sonner";

export function ToastProvider() {
    return (
        <Toaster
            position="top-right"
            toastOptions={{
                classNames: {
                    toast: "bg-zinc-900 text-zinc-100 border border-zinc-800 shadow-lg rounded-xl",
                    title: "text-sm font-medium",
                    description: "text-xs text-zinc-400",
                    actionButton:
                        "bg-zinc-700 hover:bg-zinc-600 text-zinc-100 rounded-md px-2 py-1 text-xs",
                },
            }}
        />
    );
}
