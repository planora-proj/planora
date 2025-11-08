"use client";

import { createContext, type ReactNode, useContext, useState } from "react";

export interface User {
    user_tag: string;
    username: string;
    email: string;
}

interface UserContextType {
    user: User | null;
    setUser: (user: User | null) => void;
}

type UserProviderProps = {
    userIn: User | null;
    children: ReactNode;
};

const UserContext = createContext<UserContextType | undefined>(undefined);

export function UserProvider({ children, userIn }: UserProviderProps) {
    const [user, setUser] = useState<User | null>(userIn);

    return (
        <UserContext.Provider value={{ user, setUser }}>
            {children}
        </UserContext.Provider>
    );
}

export function useUser() {
    const context = useContext(UserContext);
    if (!context) throw new Error("useUser must be used inside UserProvider");
    return context;
}
