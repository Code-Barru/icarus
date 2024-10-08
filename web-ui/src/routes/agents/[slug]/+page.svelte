<script lang="ts">
	import {
		ArrowDownUp,
		ClipboardList,
		Cpu,
		EthernetPort,
		HardDrive,
		MemoryStick,
		Monitor,
		PcCase,
		Router,
		Wifi,
		WifiOff
	} from 'lucide-svelte';
	import { ProgressBar, getModalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';
	import type { Agent, Task } from '$lib/types.js';
	import { getAgentState, getTaskState } from '$lib/state.svelte.js';
	import type { Writable } from 'svelte/store';
	import TaskRow from '$lib/components/tasks/task-row.svelte';

	export let data;

	let agents: Writable<Agent[]> = getAgentState();

	let agent: Agent | undefined;

	$: agent = $agents.find((a) => a.uuid === data.uuid);

	let taskState: Writable<Task[]> = getTaskState();

	$: tasks = $taskState.filter((task) => task.agent === data.uuid);

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
							<div class="{textColor} grid grid-cols-2 md:grid-cols-1 gap-2 md:gap-0 items-center">
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
									<TaskRow {task} />{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
<!-- !Up Right Part (Tasks) -->
