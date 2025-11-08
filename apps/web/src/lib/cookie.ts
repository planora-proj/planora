import { cookies } from "next/headers";

export async function attachCookie(setCookieHeaders: string[]) {
    if (setCookieHeaders.length > 0) {
        const cookieStore = await cookies();
        for (const rawCookie of setCookieHeaders) {
            const [pair] = rawCookie.split(";");
            const [name, value] = pair.split("=");

            cookieStore.set({
                name: name.trim(),
                value: value.trim(),
                path: "/",
                httpOnly: rawCookie.includes("HttpOnly"),
                secure: rawCookie.includes("Secure"),
                sameSite: "lax",
            });
        }
    }
}
