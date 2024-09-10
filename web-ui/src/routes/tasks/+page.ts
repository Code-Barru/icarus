import { error } from "@sveltejs/kit";
import { type Task } from "$lib/types";


export async function load({fetch}) {
    let C2_URL = import.meta.env.VITE_C2_URL;
    try {
        const response = await fetch(`${C2_URL}/tasks`);
        const tasks: Task[] = await response.json();
        return {
            tasks
        };

    } catch (er) {
        throw error(404, `Could not get tasks`)
    }
}