import type { Agent, Task } from "$lib/types";

export async function load({fetch}) {
    const C2_URL = import.meta.env.VITE_C2_URL;

    const agentResponse = await fetch(`${C2_URL}/agents`);
    const agents: Agent[] = await agentResponse.json();
    
    const taskResponse = await fetch(`${C2_URL}/tasks`);
    const tasks: Task[] = await taskResponse.json();
    return {
        agents,
        tasks
    }

}