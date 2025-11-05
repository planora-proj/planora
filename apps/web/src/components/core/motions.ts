export const motionConfig = {
    transition: { duration: 0.25, ease: [0.25, 0.1, 0.25, 1.0] },
};

export const fadeIn = {
    hidden: { opacity: 0 },
    show: { opacity: 1, transition: motionConfig.transition },
};

export const slideUp = {
    hidden: { opacity: 0, y: 20 },
    show: { opacity: 1, y: 0, transition: motionConfig.transition },
};
