<script lang="ts">
    import { onMount } from 'svelte';

    let specs: Record<string, string | number> = {};
    let temps = { CPU: [], GPU: [] } as {
        CPU: { label: string; value: number }[];
        GPU: { label: string; value: number }[];
    };

    let currentIndex = 0;
    const sections = ['PC Specifications', 'CPU Temperatures', 'GPU Temperatures'];

    function next() {
        if (currentIndex < sections.length - 1) currentIndex++;
    }

    function prev() {
        if (currentIndex > 0) currentIndex--;
    }

    async function fetchPcInfo() {
        try {
            const res = await fetch('/api/pc-info');
            if (res.ok) {
                const data = await res.json();
                specs = data.specs || {};
                temps = data.temps || { CPU: [], GPU: [] };
            }
        } catch (e) {
            console.error('Error fetching PC info', e);
        }
    }

    onMount(() => {
        fetchPcInfo();
        const interval = setInterval(fetchPcInfo, 5000);
        return () => clearInterval(interval);
    });

    function toNumber(value: string | number): number {
        return typeof value === 'number' ? value : parseFloat(value) || 0;
    }

    function usageColor(percentage: number) {
        if (percentage < 50) return '#4caf50';
        if (percentage < 75) return '#ffeb3b';
        return '#f44336';
    }
</script>

<div class="pc-info">
    <div class="carousel-wrapper">
        <!-- Triangular Arrows -->
        <button
                class="nav-arrow left"
                on:click={prev}
                disabled={currentIndex === 0}
                aria-label="Previous section"
        ></button>

        <button
                class="nav-arrow right"
                on:click={next}
                disabled={currentIndex === sections.length - 1}
                aria-label="Next section"
        ></button>


        <!-- Carousel Content -->
        <div class="carousel-viewport">
            <div class="carousel-content" style="transform: translateX(-{currentIndex * 100}%);">
                <!-- Section 1 -->
                <section>
                    <h2>PC Specifications</h2>
                    <ul class="specs-list">
                        {#each Object.entries(specs) as [key, value]}
                            <li>
                                <span class="label">{key}</span>
                                <span class="value">{value}</span>

                                {#if key === 'CPU Load %'}
                                    <div class="progress-bar"
                                         style="--progress: {toNumber(value)}%; --progress-color: {usageColor(toNumber(value))};"
                                         role="progressbar" aria-valuemin="0" aria-valuemax="100"
                                         aria-valuenow={toNumber(value)}></div>
                                {:else if key === 'RAM Used (GB)' && specs['RAM Total (GB)']}
                                    <div class="progress-bar"
                                         style="--progress: {(toNumber(value) / toNumber(specs['RAM Total (GB)'])) * 100}%;
											   --progress-color: {usageColor((toNumber(value) / toNumber(specs['RAM Total (GB)'])) * 100)};"
                                         role="progressbar"
                                         aria-valuemin="0"
                                         aria-valuemax={toNumber(specs['RAM Total (GB)'])}
                                         aria-valuenow={toNumber(value)}></div>
                                {/if}
                            </li>
                        {/each}
                    </ul>
                </section>

                <!-- Section 2 -->
                <section>
                    <h2>CPU Temperatures</h2>
                    {#if temps.CPU.length > 0}
                        <ul class="temp-list">
                            {#each temps.CPU as temp}
                                <li>
                                    <span class="label">{temp.label}</span>
                                    <span class="value">{temp.value.toFixed(2)}</span>
                                    <div class="progress-bar"
                                         style="--progress: {Math.min(temp.value, 100)}%; --progress-color: {usageColor(temp.value)};"
                                         role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow={temp.value}></div>
                                </li>
                            {/each}
                        </ul>
                    {:else}
                        <p class="empty-msg">No CPU temperature sensors detected.</p>
                    {/if}
                </section>

                <!-- Section 3 -->
                <section>
                    <h2>GPU Temperatures</h2>
                    {#if temps.GPU.length > 0}
                        <ul class="temp-list">
                            {#each temps.GPU as temp}
                                <li>
                                    <span class="label">{temp.label}</span>
                                    <span class="value">{temp.value.toFixed(2)}</span>
                                    <div class="progress-bar"
                                         style="--progress: {Math.min(temp.value, 100)}%; --progress-color: {usageColor(temp.value)};"
                                         role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow={temp.value}></div>
                                </li>
                            {/each}
                        </ul>
                    {:else}
                        <p class="empty-msg">No GPU temperature sensors detected.</p>
                    {/if}
                </section>
            </div>
        </div>

        <!-- Dots -->
        <div class="dots">
            {#each sections as _, index}
                <span class:active={index === currentIndex}></span>
            {/each}
        </div>
    </div>
</div>

<style>
    .carousel-wrapper {
        position: relative;
        padding: 0 2rem;
    }

    .carousel-viewport {
        overflow: hidden;
        border-radius: 8px;
    }

    .carousel-content {
        display: flex;
        width: 100%;
        transition: transform 0.4s ease;
    }

    section {
        min-width: 100%;
        padding: 1.5rem;
        box-sizing: border-box;
    }

    h2 {
        font-size: 1rem;
        font-weight: 600;
        margin-bottom: 0.8rem;
        color: #fff;
        border-bottom: 1px solid #333;
        padding-bottom: 0.4rem;
    }

    ul {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .specs-list {
        column-count: 2;
        column-gap: 1.5rem;
    }

    .specs-list li,
    .temp-list li {
        break-inside: avoid;
        padding: 0.3rem 0;
        border-bottom: 1px solid #2a2a2a;
        font-size: 0.8rem;
        font-family: "Roboto Mono", monospace;
        display: flex;
        flex-direction: column;
    }

    .label {
        color: #aaa;
    }

    .value {
        color: #fff;
        font-weight: 500;
    }

    .empty-msg {
        color: #777;
        font-style: italic;
        font-size: 0.75rem;
    }

    .progress-bar {
        height: 5px;
        width: 100%;
        background: #222;
        border-radius: 2px;
        margin-top: 3px;
        position: relative;
        overflow: hidden;
    }

    .progress-bar::before {
        content: '';
        position: absolute;
        height: 100%;
        left: 0;
        top: 0;
        width: var(--progress, 0%);
        background-color: var(--progress-color, #4caf50);
        transition: width 0.5s ease;
    }

    .nav-arrow {
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        background: transparent;
        border: none;
        cursor: pointer;
        width: 24px;
        height: 24px;
        padding: 0;
        z-index: 10;
    }
    .nav-arrow:disabled {
        opacity: 0.3;
        cursor: not-allowed;
    }
    /* Left arrow */
    .nav-arrow.left::before {
        content: '';
        display: block;
        margin: auto;
        width: 0;
        height: 0;
        border-top: 12px solid transparent;
        border-bottom: 12px solid transparent;
        border-right: 12px solid white; /* arrow color */
    }
    /* Right arrow */
    .nav-arrow.right::before {
        content: '';
        display: block;
        margin: auto;
        width: 0;
        height: 0;
        border-top: 12px solid transparent;
        border-bottom: 12px solid transparent;
        border-left: 12px solid white; /* arrow color */
    }
    .nav-arrow.left {
        left: 8px;
    }
    .nav-arrow.right {
        right: 8px;
    }
    .dots {
        display: flex;
        justify-content: center;
        gap: 0.4rem;
        margin: 1rem 0;
    }

    .dots span {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        background: #666;
        transition: background 0.3s;
    }

    .dots span.active {
        background: #fff;
    }
</style>
