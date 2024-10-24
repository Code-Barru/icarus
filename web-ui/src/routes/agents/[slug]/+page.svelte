<script lang="ts">
	import {
		ArrowDownUp,
		ClipboardList,
		Cpu,
		EthernetPort,
		Folders,
		HardDrive,
		House,
		MemoryStick,
		Monitor,
		MoveLeft,
		PcCase,
		Router,
		RotateCcw,
		SquareTerminal,
		Wifi,
		WifiOff
	} from 'lucide-svelte';
	import { ProgressBar, getModalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';
	import { TaskType, type Agent, type Task } from '$lib/types.js';
	import {
		getAgentState,
		getTaskState,
		getExplorerState,
		updateAgentExplorerPath,
		getDirectoryState
	} from '$lib/state.svelte.js';
	import type { Writable } from 'svelte/store';
	import TaskRow from '$lib/components/tasks/task-row.svelte';
	import ExplorerWrapper from '$lib/components/explorer/wrapper.svelte';
	const C2_URL = `http://localhost:1337`;

	export let data;

	let agents: Writable<Agent[]> = getAgentState();
	let taskState: Writable<Task[]> = getTaskState();
	let explorerState = getExplorerState();
	let directoryState = getDirectoryState();

	let agent: Agent | undefined;
	let tasks: Task[] | undefined;

	$: agent = $agents.find((a) => a.uuid === data.uuid);
	$: tasks = $taskState.filter((task) => task.agent === data.uuid);
	$: explorer = $explorerState.find((e) => e.agent === data.uuid);

	const modalStore = getModalStore();
	const createTask: ModalSettings = {
		title: 'Create Task',
		component: 'createTask',
		meta: {
			agent: data.uuid
		},
		type: 'component'
	};

	let textColor = 'text-primary-900-50-token';

	function updateExplorerPath(path: string) {
		path = path.replace('\\', '/');
		updateAgentExplorerPath(data.uuid, path);
		if (!$directoryState.find((d) => d.agent === data.uuid && d.path === path)) {
			fetch(`${C2_URL}/explorer/${data.uuid}?path=${path}`).catch((error) => {
				console.error('Error:', error);
			});
		}
	}

	function explorerBack() {
		//@ts-ignore
		const path = explorer.path.split('/');
		path.pop();
		updateAgentExplorerPath(data.uuid, path.join('/'));
		if (!$directoryState.find((d) => d.agent === data.uuid && d.path === path.join('/'))) {
			fetch(`${C2_URL}/explorer/${data.uuid}?path=${path.join('/')}`).catch((error) => {
				console.error('Error:', error);
			});
		}
	}
</script>

{#if agent}
	<div class="flex flex-col lg:flex-row h-full">
		<!-- Left part (Infos) -->
		<div class="h-full flex w-full lg:w-80 text-primary-100">
			<div
				class="flex-grow px-2 bg-surface-100-800-token mx-4 my-2 border border-surface-800-100-token rounded-lg"
			>
				<span
					class=" {textColor} text-inherit text-2xl flex flex-row my-4 justify-center font-bold"
				>
					<Monitor class="w-8 h-8 mr-2" />
					{agent.hostname}</span
				>
				<hr />
				<div class="flex flex-col flex-grow">
					<div>
						<span class="{textColor} text-xl flex flex-row my-4 justify-start font-bold">
							<PcCase class="w-7 h-7 mr-2" />
							Hardware
						</span>
						<div class="{textColor} pl-4 text-sm justify-start">
							<div class="flex flex-row my-">
								<Cpu class="w-5 h-5 mr-2" />
								{agent.hardware.cpu}
							</div>
							<div class="{textColor} flex flex-row my-1">
								<MemoryStick class="w-5 h-5 mr-2" />
								{Number(Number(agent.hardware.memory) / 1024 / 1024 / 1024).toPrecision(2)} GB
							</div>
						</div>
					</div>
					<div>
						<span class="{textColor} text-xl flex flex-row my-4 justify-start font-bold">
							<ArrowDownUp class="w-7 h-7 mr-2" />
							Network
						</span>
						<div class="pl-4">
							<div class="{textColor} flex flex-row my-1">
								{#if agent.status === 'Online'}
									<Wifi class="w-5 h-5 mr-2 stroke-success-500" />
								{:else}
									<WifiOff class="w-5 h-5 mr-2 stroke-error-500" />
								{/if}
								<p
									class={agent.status.toString() === 'Online'
										? 'text-success-500'
										: 'text-error-500'}
								>
									{agent.status}
								</p>
							</div>
							<div class="{textColor} flex flex-row my-1">
								<Router class="w-5 h-5 mr-2" />
								{agent.ip}
							</div>
							<div class="{textColor} flex flex-row my-1">
								<EthernetPort class="w-5 h-5 mr-2" />
								{agent.hardware.mac_address}
							</div>
						</div>
					</div>
					<div>
						<span class="{textColor} text-xl flex flex-row my-4 justify-start font-bold">
							<HardDrive class="w-7 h-7 mr-2" />
							Storage
						</span>
						<div class="{textColor} px-4 flex flex-col text-center max-h-72 overflow-y-auto">
							{#each agent.hardware.disks as drive}
								<div class="my-2">
									<div class="flex flex-row justify-between">
										{drive.name} ({drive.mount_point})
									</div>
									<ProgressBar
										value={Number(drive.total) - Number(drive.free)}
										max={Number(drive.total)}
										meter={(Number(drive.used) / Number(drive.total)) * 100 > 90
											? 'bg-error-500'
											: 'bg-primary-500'}
										track={(Number(drive.used) / Number(drive.total)) * 100 > 90
											? 'bg-error-500/30'
											: 'bg-primary-500/30'}
									/>
									{Number(Number(drive.free) / 1024 / 1024 / 1024).toPrecision(4)} GB free of {Number(
										Number(drive.total) / 1024 / 1024 / 1024
									).toPrecision(4)} GB
								</div>
								<hr />
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>
		<!-- !Left part (Infos) -->
		<!-- Up Right Part (Tasks) -->
		<div class="flex flex-col">
			<div class="md:h-96 flex w-full text-primary-100">
				<div
					class="flex-grow px-2 bg-surface-100-800-token mx-4 my-2 border border-surface-800-100-token rounded-lg"
				>
					<div class="flex flex-row justify-between">
						<div class="{textColor} px-2 text-2xl flex flex-row my-4 justify-start font-bold">
							<ClipboardList class="w-8 h-8 mr-2" />
							Tasks
						</div>
						<button
							class="{textColor} btn btn-sm variant-ghost-primary h-8 w-8 mt-4 mr-2"
							on:click={() => modalStore.trigger(createTask)}>+</button
						>
					</div>
					<hr />
					<div class="flex flex-grow md:flex-row flex-col my-2 max-w-full md:max-h-72">
						<div class="w-full md:min-w-36 md:w-fit h-full mx-2 pb-4 md:pb-0">
							<div>
								<div
									class="{textColor} grid grid-cols-2 md:grid-cols-1 gap-2 md:gap-0 items-center"
								>
									<div class="flex flex-col items-center">
										<div class="text-sm mt-2">Pending Tasks</div>
										<div class="text-xl font-bold">
											{tasks.filter((task) => task.status.toString() === 'Pending').length}
										</div>
									</div>
									<div class="flex flex-col items-center">
										<div class="text-sm mt-2">In Progress Tasks</div>
										<div class="text-xl font-bold text-secondary-500">
											{tasks.filter((task) => task.status.toString() === 'InProgress').length}
										</div>
									</div>
									<div class="flex flex-col items-center">
										<div class="text-sm mt-2">Failed Tasks</div>
										<div class="text-xl font-bold text-error-500">
											{tasks.filter((task) => task.status.toString() === 'Failed').length}
										</div>
									</div>
									<div class="flex flex-col items-center">
										<div class="text-sm mt-2">Completed Tasks</div>
										<div class="text-xl font-bold text-success-500">
											{tasks.filter((task) => task.status.toString() === 'Completed').length}
										</div>
									</div>
									<div class="flex flex-col items-center">
										<div class="text-sm mt-2">Total Tasks:</div>
										<div class="text-xl font-bold">{agent.tasks.length}</div>
									</div>
								</div>
							</div>
						</div>
						<hr class=" md:hidden py-2" />
						<span class="hidden md:block divider-vertical h-72 class m-0 p-0"></span>
						<div class="{textColor} w-full mx-2 table-container overflow-x-auto">
							<table class="table table-fixed table-hover min-w-fit">
								<thead class="sticky top-0 z-10">
									<tr>
										<th>Task</th>
										<th>Status</th>
										<th class="hidden md:table-cell">Created At</th>
										<th class="hidden md:table-cell">Completed At</th>
									</tr>
								</thead>
								<tbody>
									{#each tasks as task}
										{#if task.task_type !== TaskType.Explorer}
											<TaskRow {task} />
										{/if}
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				</div>
			</div>
			<div class="flex lg:flex-row flex-col lg:h-full">
				<div class="lg:h-full flex-grow flex w-full lg:w-8/12 text-primary-100">
					<div
						class="flex-grow px-2 bg-surface-100-800-token mx-4 my-2 border border-surface-800-100-token rounded-lg"
					>
						<div class="{textColor} px-2 text-2xl flex flex-row my-4 justify-between font-bold">
							<div class="flex flex-row">
								<Folders class="w-8 h-8 mr-2" />
								Explorer
							</div>
							<div class="flex flex-rox">
								{#if explorer && explorer.path.length > 0}
									<button on:click={() => explorerBack()}>
										<MoveLeft class="w-8 h-8 mr-2" />
									</button>
								{/if}
								<button on:click={() => updateAgentExplorerPath(agent.uuid, '')}>
									<House class="w-8 h-8 mr-2" />
								</button>
							</div>
						</div>
						<hr />
						<div class="my-2 mx-2">
							{#if explorer && explorer.path.length > 0}
								<div class="flex flex-row">
									<HardDrive class="w-8 h-8 my-2 mr-2" />
									{explorer.path}
								</div>
								<div class="max-h-96 my-3 overflow-y-auto">
									<ExplorerWrapper path={explorer.path} agent={agent.uuid} />
								</div>
							{:else}
								<div class="flex flex-col justify-center text-xl text-gray-200 font-semibold">
									Select a disk to explore
									<div class="grid grid-cols-2 gap-4 mt-4">
										{#each agent.hardware.disks as disk}
											<button
												class="flex flex-col items-center text-primary-100 my-2 p-2 bg-surface-100-800-token rounded-lg"
												on:click={() => updateExplorerPath(disk.mount_point)}
											>
												<HardDrive class="w-8 h-8 mr-2" />

												{disk.name} ({disk.mount_point})
											</button>
										{/each}
									</div>
								</div>
							{/if}
						</div>
					</div>
				</div>
				<div class="h-96 lg:h-full flex-grow flex w-full lg:w-4/12 text-primary-100">
					<div
						class="flex-grow px-2 bg-surface-100-800-token mx-4 my-2 border border-surface-800-100-token rounded-lg"
					>
						<div class="{textColor} px-2 text-2xl flex flex-row my-4 justify-start font-bold">
							<SquareTerminal class="w-8 h-8 mr-2" />
							Reverse Shell
						</div>
						<hr />
						<div class="flex justify-center text-xl text-gray-200 font-semibold">
							Not done yet :(
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
<!-- !Up Right Part (Tasks) -->
