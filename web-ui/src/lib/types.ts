export type Agent = {
    uuid: string;
    status: AgentStatus;
    tasks: Task[];
    ip: string;
    hostname: string;
    platform: string;

    createdAt: string;
    last_seen_at: string;
}

export enum AgentStatus {
    Online = 'Online',
    Offline = 'Offline',
}

export type Task = {
    uuid: string;
    task_type: TaskType;
    agent: string;
    status: TaskStatus;
    response: string;
    input: string;

    emitted_at: string;
    completed_at: string;
}

export enum TaskType  {
    ShellCommand = 'ShellCommand',
    PowerShellCommand = 'PowerShellCommand'
}

export enum TaskStatus {
    Pending = 'Pending',
    InProgress = 'InProgress',
    Completed = 'Completed',
    Failed = 'Failed'
}