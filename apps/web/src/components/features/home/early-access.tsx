"use client";

import { motion } from "motion/react";
import { Great_Vibes, Poppins } from "next/font/google";
import Link from "next/link";
import { Button } from "@/components/ui/button";

const poppins = Poppins({
    subsets: ["latin"],
    weight: ["700"],
});

const greatVibes = Great_Vibes({
    subsets: ["latin"],
    weight: ["400"],
});

const fadeUp = {
    hidden: { opacity: 0, y: 20 },
    show: { opacity: 1, y: 0, transition: { duration: 0.6 } },
};

export function EarlyAccessNotice() {
    return (
        <motion.div
            initial="hidden"
            animate="show"
            variants={fadeUp}
            className={`${poppins.className} max-w-5xl mx-auto my-8 mb-24 px-4`}
        >
            <div className="flex bg-gradient-to-r from-indigo-600/20 via-purple-600/10 to-pink-600/20 rounded-2xl shadow-lg border border-muted-foreground/30 overflow-hidden">
                <div className="w-2 bg-indigo-600 rounded-l-2xl"></div>

                <div className="flex-1 p-8 md:p-12">
                    <h2 className="text-4xl font-extrabold text-foreground mb-4">
                        Welcome to{" "}
                        <span
                            className={`${greatVibes.className} text-5xl pl-2 font-bold bg-linear-to-r from-indigo-500 via-purple-500 to-pink-500 bg-clip-text text-transparent`}
                        >
                            Planora
                        </span>
                    </h2>
                    <p className="text-lg text-muted-foreground mb-4">
                        Planora is a modern team collaboration tool that helps
                        teams manage projects, tasks, meetings, and
                        communication in one central place.
                    </p>
                    <p className="text-lg text-muted-foreground mb-6">
                        This is an early stage release. Features are limited and
                        evolving. We greatly appreciate your feedback and
                        support to help us build a better tool.
                    </p>

                    <div className="flex justify-center gap-4">
                        <Button asChild variant="default">
                            <Link
                                href="https://github.com/planora-proj/planora/issues"
                                target="_blank"
                            >
                                Give Feedback
                            </Link>
                        </Button>
                        <Button asChild variant="outline">
                            <Link
                                href="https://github.com/planora-proj/planora/discussions"
                                target="_blank"
                            >
                                Join Discussion
                            </Link>
                        </Button>
                    </div>
                </div>
            </div>
        </motion.div>
    );
}
