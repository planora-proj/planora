import type { Variants } from "motion";

export const fadeIn: Variants = {
    hidden: {
        opacity: 0,
        transition: {
            duration: 0.25,
        },
    },
    show: {
        opacity: 1,
        transition: {
            duration: 0.25,
        },
    },
};

export const slideUp: Variants = {
    hidden: { opacity: 0, y: 20 },
    show: { opacity: 1, y: 0, transition: { duration: 0.3 } },
};

export const slideLeft: Variants = {
    hidden: { opacity: 0, x: 20 },
    show: { opacity: 1, x: 0, transition: { duration: 0.3 } },
};

export const slideRight: Variants = {
    hidden: { opacity: 0, x: -20 },
    show: { opacity: 1, x: 0, transition: { duration: 0.3 } },
};

export const scaleIn: Variants = {
    hidden: { opacity: 0, scale: 0.95 },
    show: { opacity: 1, scale: 1, transition: { duration: 0.3 } },
};
