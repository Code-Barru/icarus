import { AgentStatus, type Agent, type Directory, type Task } from "./types";
import { writable, type Writable } from "svelte/store";

const agentState: Writable<Agent[]> = writable();

export function setAgentState(agents: Agent[]) {
    agentState.set(agents);
    return agentState;
}

export function getAgentState() {
    return agentState;
}

export function addAgent(agent: Agent) {
    getAgentState().update(agents => [...agents, agent]);
}

export function updateAgent(agent: Agent) {
    getAgentState().update(agents => agents.map(a => a.uuid === agent.uuid ? agent : a));
    getAgentState().update(agents => {
        return agents.sort((a, b) => {
            if (a.status.toString() === 'Offline' && b.status.toString() !== 'Offline') return 1;
            if (a.status.toString() !== 'Offline' && b.status.toString() === 'Offline') return -1;
            return 0;
        });
    });
}

export function agentDisconnect(uuid: string) {
    getAgentState().update(agents => agents.map(a => a.uuid === uuid ? { ...a, status: AgentStatus.Offline } : a));
}

const taskState: Writable<Task[]> = writable();

export function setTaskState(tasks: Task[]) {
    taskState.set(tasks);
    return taskState;
}

export function getTaskState() {
    return taskState;
}

export function addTask(task: Task) {
    getTaskState().update(tasks => [...tasks, task]);
    getAgentState().update(agents => {
        return agents.map(agent => {
            if (agent.uuid === task.agent) {
                return { ...agent, tasks: [...agent.tasks, task.uuid] };
            }
            return agent;
        });
    });
}

export function updateTask(task: Task) {
    getTaskState().update(tasks => tasks.map(t => t.uuid === task.uuid ? task : t));
}

const directoryState: Writable<Directory[]> = writable();

export function setDirectoryState(directories: Directory[]) {
    directoryState.set(directories);
    return directoryState;
}

export function getDirectoryState() {
    return directoryState;
}

export function addDirectory(directory: Directory) {
    getDirectoryState().update(directories => [...directories, directory]);
}

export function updateDirectory(directory: Directory) {
    getDirectoryState().update(directories => directories.map(d => (d.agent === directory.agent && d.path === directory.path) ? directory : d));
}

type ExplorerAgent = {
    agent: string,
    path: string,
    viewHidden: boolean,
}

const explorerState: Writable<ExplorerAgent[]> = writable();

export function setExplorerState(state: ExplorerAgent[]) {
    explorerState.set(state);
}

export function updateAgentExplorerPath(agent: string, path: string) {
    explorerState.update(state => {
        return state.map(e => e.agent === agent ? { ...e, path } : e);
    });
}

export function addExplorerAgent(agent: string, path: string, viewHidden: boolean) {
    explorerState.update(state => {
        return [...state, { agent, path, viewHidden }]; 
    });
}

export function removeExplorerAgent(agent: string) {
    explorerState.update(state => {
        return state.filter(e => e.agent !== agent);
    });
}

export function getExplorerState() {
    return explorerState;
}