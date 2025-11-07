import { cookies } from "next/headers";

import { config } from "@/lib/config";

export async function fetchUser() {
    try {
        const cookieHeader = (await cookies())
            .getAll()
            .map((c) => `${c.name}=${c.value}`)
            .join("; ");

        const res = await fetch(`${config.api}/v1/auth/profile`, {
            headers: { Cookie: cookieHeader },
            credentials: "include",
            cache: "no-store",
        });

        if (!res.ok) return null;

        const data = await res.json();
        return data.user ?? null;
    } catch (err) {
        return null;
    }
}
