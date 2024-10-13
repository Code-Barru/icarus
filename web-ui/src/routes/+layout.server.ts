import type { Agent, Task } from "$lib/types";

export async function load({fetch}) {
    const C2_URL = import.meta.env.VITE_C2_SERVER_URL;

    const agentResponse = await fetch(`${C2_URL}/agents`);
    const agents: Agent[] = await agentResponse.json();
    
    const taskResponse = await fetch(`${C2_URL}/tasks`);
    const tasks: Task[] = await taskResponse.json();

    const directoryResponse = await fetch(`${C2_URL}/explorer`);
    const directories = await directoryResponse.json();

    return {
        agents,
        tasks,
        directories
    }

}