import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async () => {
    try {
        const res = await fetch("http://localhost:8000/api/pc-info");
        if (!res.ok) {
            console.error(`Fetch failed with status ${res.status}`);
            return new Response("Failed to fetch PC info", { status: res.status });
        }

        const data = await res.json();
        return new Response(JSON.stringify(data), {
            status: 200,
            headers: { "Content-Type": "application/json" },
        });
    } catch (error) {
        console.error("Error fetching PC info:", error);
        return new Response("Failed to fetch PC info", { status: 500 });
    }
};
