<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { isRequestActive } from "../stores/request_status";
    import { type Unsubscriber } from "svelte/store";

    let container: HTMLDivElement;

    let containerSize = 0;
    let center = 0;

    interface DotData {
        dot: HTMLDivElement;
        angle: number;
        baseSpeed: number;
        currentSpeed: number;
        radius: number;
        currentRadius: number;
        targetRadius: number;
        ringIndex: number;
        spinningFrozen: boolean;
    }

    const rings = [
        { radius: 50, dots: 6, baseSpeed: 0.02 },
        { radius: 90, dots: 14, baseSpeed: 0.015 },
        { radius: 140, dots: 28, baseSpeed: 0.01 },
    ];

    let dotsData: DotData[] = [];
    let ringGroups: number[][] = [];

    let collapsedToSecondRing = true;
    let waveActive = false;
    let waveStartTime = 0;
    const waveDelay = 100;

    let waveIntervalId: number | null = null;

    let svg: SVGSVGElement;
    let glowRing: SVGPathElement;
    const secondRingRadius = rings[1].radius;
    let lastGlowRingIndex = -1;
    let glowRadius = rings[rings.length - 1].radius;
    let targetGlowRadius = glowRadius;

    function lerp(a: number, b: number, t: number) {
        return a + (b - a) * t;
    }

    function getSlimePath(radius: number, time: number) {
        const pointsCount = 32;
        const baseRadiusOffset = 4;
        let path = "";
        const angleStep = (Math.PI * 2) / pointsCount;

        for (let i = 0; i < pointsCount; i++) {
            const angle = i * angleStep;
            const wobble =
                Math.sin(time * 0.005 + i) * 3 +
                Math.sin(time * 0.01 + i * 1.5) * 2 +
                Math.sin(time * 0.02 + i * 0.7) * 1.5;
            const r = radius + baseRadiusOffset + wobble;
            const x = center + r * Math.cos(angle);
            const y = center + r * Math.sin(angle);
            path += i === 0
                ? `M ${x.toFixed(2)} ${y.toFixed(2)} `
                : `L ${x.toFixed(2)} ${y.toFixed(2)} `;
        }
        path += "Z";
        return path;
    }

    function animate(now: number) {
        glowRadius = lerp(glowRadius, targetGlowRadius, 0.1);
        glowRing.setAttribute("d", getSlimePath(glowRadius, now));
        glowRing.setAttribute("opacity", waveActive || collapsedToSecondRing ? "1" : "0");

        dotsData.forEach((d) => {
            if (!d.spinningFrozen) {
                d.angle += d.currentSpeed;
                if (d.angle > Math.PI * 2) d.angle -= Math.PI * 2;
            }
            d.currentRadius = lerp(d.currentRadius, d.targetRadius, 0.05);
            const x = center + d.currentRadius * Math.cos(d.angle);
            const y = center + d.currentRadius * Math.sin(d.angle);
            d.dot.style.left = `${x}px`;
            d.dot.style.top = `${y}px`;
        });

        if (waveActive) {
            let allRingsDone = true;
            let maxActiveRingIndex = -1;

            ringGroups.forEach((group, ringIndex) => {
                const elapsed = now - waveStartTime;
                const triggerTime = ringIndex * waveDelay;

                if (elapsed >= triggerTime && elapsed < triggerTime + 300) {
                    group.forEach((dotIndex) => {
                        const d = dotsData[dotIndex];
                        const progress = (elapsed - triggerTime) / 300;
                        const scale = 1 + 2 * Math.sin(progress * Math.PI);
                        d.dot.style.transform = `translate(-50%, -50%) scale(${scale.toFixed(2)})`;
                    });

                    if (lastGlowRingIndex < ringIndex) {
                        targetGlowRadius = rings[ringIndex].radius;
                        lastGlowRingIndex = ringIndex;
                    }

                    maxActiveRingIndex = Math.max(maxActiveRingIndex, ringIndex);
                    allRingsDone = false;
                } else if (elapsed < triggerTime) {
                    allRingsDone = false;
                } else {
                    group.forEach((dotIndex) => {
                        const d = dotsData[dotIndex];
                        d.dot.style.transform = "translate(-50%, -50%) scale(1)";
                    });
                }
            });

            dotsData.forEach((d) => {
                if (!d.spinningFrozen) {
                    const accelerationFactor = 1;
                    const dragFactor = 0.85;
                    const targetSpeed =
                        d.ringIndex <= maxActiveRingIndex && maxActiveRingIndex !== -1
                            ? d.baseSpeed * 2
                            : d.baseSpeed;

                    const acceleration = (targetSpeed - d.currentSpeed) * accelerationFactor;
                    d.currentSpeed += acceleration;
                    d.currentSpeed *= dragFactor;
                }
            });

            if (allRingsDone) {
                waveActive = false;
                lastGlowRingIndex = -1;
                dotsData.forEach((d) => {
                    if (!d.spinningFrozen) d.currentSpeed = d.baseSpeed;
                });
            }
        }

        requestAnimationFrame(animate);
    }

    function setupDots() {
        let index = 0;
        rings.forEach(({ radius, dots, baseSpeed }, ringIndex) => {
            const group: number[] = [];
            for (let i = 0; i < dots; i++) {
                const angle = (360 / dots) * i * (Math.PI / 180);
                const dot = document.createElement("div");
                dot.classList.add("dot");
                dot.style.position = "absolute";
                dot.style.width = "8px";
                dot.style.height = "8px";
                dot.style.backgroundColor = "#5d0891";
                dot.style.borderRadius = "50%";
                dot.style.transform = "translate(-50%, -50%)";
                container.appendChild(dot);

                dotsData.push({
                    dot,
                    angle,
                    baseSpeed,
                    currentSpeed: baseSpeed,
                    radius,
                    currentRadius: radius,
                    targetRadius: radius,
                    ringIndex,
                    spinningFrozen: false,
                });

                group.push(index++);
            }
            ringGroups.push(group);
        });
    }

    function setupSVG() {
        svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("width", containerSize.toString());
        svg.setAttribute("height", containerSize.toString());
        container.appendChild(svg);

        glowRing = document.createElementNS("http://www.w3.org/2000/svg", "path");
        glowRing.setAttribute("fill", "none");
        glowRing.setAttribute("stroke", "rgba(93, 8, 145, 0.5)");
        glowRing.setAttribute("stroke-width", "8");
        glowRing.style.filter = "drop-shadow(0 0 10px #5d0891) drop-shadow(0 0 20px #5d0891)";
        svg.appendChild(glowRing);
    }

    let unsubscribe: Unsubscriber | null = null;

    onMount(() => {
        const rect = container.getBoundingClientRect();
        containerSize = rect.width;
        center = containerSize / 2;
        setupSVG();
        setupDots();

        dotsData.forEach((d) => {
            d.targetRadius = secondRingRadius;
            d.spinningFrozen = false;
            d.currentSpeed = d.baseSpeed;
        });

        targetGlowRadius = secondRingRadius;
        glowRadius = secondRingRadius;

        animate(performance.now());

        unsubscribe = isRequestActive.subscribe((active) => {
            if (active) {
                collapsedToSecondRing = false;

                dotsData.forEach((d) => {
                    d.targetRadius = rings[d.ringIndex].radius;
                    d.currentSpeed = 0;
                    d.dot.style.transform = "translate(-50%, -50%) scale(1)";
                });

                waveActive = true;
                waveStartTime = performance.now();
                lastGlowRingIndex = -1;

                if (waveIntervalId === null) {
                    waveIntervalId = setInterval(() => {
                        if (!collapsedToSecondRing && !waveActive) {
                            waveActive = true;
                            waveStartTime = performance.now();
                            lastGlowRingIndex = -1;
                        }
                    }, 4000);
                }
            } else {
                collapsedToSecondRing = true;

                dotsData.forEach((d) => {
                    d.targetRadius = secondRingRadius;
                    d.spinningFrozen = false;
                    d.currentSpeed = d.baseSpeed;
                });

                targetGlowRadius = secondRingRadius;
                glowRadius = secondRingRadius; // 🔥 FIX applied here

                if (waveIntervalId !== null) {
                    clearInterval(waveIntervalId);
                    waveIntervalId = null;
                }
            }
        });
    });

    onDestroy(() => {
        if (unsubscribe) unsubscribe();
        if (waveIntervalId !== null) {
            clearInterval(waveIntervalId);
            waveIntervalId = null;
        }
    });
</script>


<div class="container" bind:this={container}></div>

<style>
    .container {
        position: relative;
        width: 100%;
        aspect-ratio: 1 / 1;
        margin: 0 auto;
    }

</style>
