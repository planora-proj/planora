import type { ReactNode } from "react";
import { type User, UserProvider } from "./user-context";

type AppProviderProps = {
    user: User | null;
    children: ReactNode;
};

export function AppProvider({ user, children }: AppProviderProps) {
    return <UserProvider userIn={user}>{children}</UserProvider>;
}
