"use client";

import { EarlyAccessNotice } from "@/components/features/home/early-access";
import { HeroSection } from "@/components/features/home/hero";
import { Welcome } from "@/components/features/home/welcome";
import { Footer } from "@/components/layout/footer";
import { Navbar } from "@/components/layout/navbar";
import { useUser } from "@/context/user-context";

export default function HomePage() {
    const { user } = useUser();

    return (
        <div className="min-h-screen bg-background text-foreground">
            <Navbar />

            <main className="py-24 px-6 md:px-12">
                <EarlyAccessNotice />

                {user ? (
                    <section className="mb-16">
                        <Welcome name={user.username} />
                    </section>
                ) : null}

                <HeroSection />
            </main>

            <Footer />
        </div>
    );
}
