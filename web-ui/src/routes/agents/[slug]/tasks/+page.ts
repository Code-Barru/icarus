import type { Task } from '$lib/types.js';

export async function load({ fetch, params }) {
    let C2_URL = import.meta.env.VITE_C2_URL;
    const response = await fetch(`${C2_URL}/agents/${params.slug}/tasks`);
    const tasks: Task[] = await response.json();
    return {
        props: {
            tasks,
        }
    };
}