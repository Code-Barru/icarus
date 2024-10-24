export type Agent = {
    uuid: string;
    status: AgentStatus;
    tasks: string[];
    ip: string;
    hostname: string;
    platform: string;
    hardware: AgentHardware;

    createdAt: string;
    last_seen_at: string;
}

export type AgentHardware = {
    cpu: string,
    memory: BigUint64Array,
    disks: AgentDisk[],
    mac_address: string,
}

export type AgentDisk = {
    total: BigInt,
    free: BigInt,
    used: BigInt,
    name: string,
    mount_point: string,
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
    Shell = 'Shell',
    Explorer = 'Explorer',
    FileUpload = 'FileUpload',
    FileDownload = 'FileDownload'
}

export enum TaskStatus {
    Pending = 'Pending',
    InProgress = 'InProgress',
    Completed = 'Completed',
    Failed = 'Failed'
}

export type Directory = {
    agent: string;
    path: string;
    files: File[] | undefined;
}

export type File = {
    name: string;
    size: number;
    is_dir: boolean;
    created_at: string;
    modified_at: string;
}