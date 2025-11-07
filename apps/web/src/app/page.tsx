"use client";

import { Navbar } from "@/components/layout/navbar";
import { Footer } from "@/components/layout/footer";
import { HeroSection } from "@/components/features/home/hero";

export default function HomePage() {
    return (
        <div className="min-h-screen bg-background text-foreground">
            <Navbar />

            <main className="py-24 pt-40 px-6 md:px-12">
                <HeroSection />
            </main>

            <Footer />
        </div>
    );
}
