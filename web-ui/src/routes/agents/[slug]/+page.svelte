<script lang="ts">
	import {
		ArrowDownUp,
		ClipboardList,
		Cpu,
		EthernetPort,
		HardDrive,
		Microchip,
		MemoryStick,
		Monitor,
		PcCase,
		Router,
		Wifi,
		WifiOff
	} from 'lucide-svelte';
	import { getDiskPourcentage } from '$lib/utils.js';
	import { ProgressBar, getModalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';

	export let data;
	let agent = data.agent;

	let agent_hardware = {
		cpu: 'Intel Core i7-10700K',
		gpu: 'NVIDIA GeForce RTX 3080',
		mac_address: 'FE:A3:C9:11:62:D2',
		ram: '32GB',
		storage: [
			{
				model: 'Samsung 970 EVO Plus 1TB',
				mountPoint: 'C:/',
				name: 'Windows',
				maxCapacity: '1TB',
				usedCapacity: '650GB',
				availableCapacity: '350GB'
			},
			{
				model: 'Samsung 970 EVO Plus 1TB',
				mountPoint: 'D:/',
				name: 'Data',
				maxCapacity: '1TB',
				usedCapacity: '350 GB',
				availableCapacity: '650 GB'
			}
		]
	};

	const modalStore = getModalStore();
	const modal: ModalSettings = {
		title: 'Create Task',
		component: 'createTask',
		meta: {
			agent: agent.uuid
		},
		type: 'component'
	};

	let textColor = 'text-primary-900-50-token';
</script>

<div class="flex flex-row h-full">
	<!-- Left part (Infos) -->
	<div class="h-full flex w-80 text-primary-100">
		<div
			class="flex-grow px-2 bg-surface-100-800-token mx-4 my-2 border border-surface-800-100-token rounded-lg"
		>
			<span class=" {textColor} text-inherit text-2xl flex flex-row my-4 justify-center font-bold">
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
							{agent_hardware.cpu}
						</div>
						<div class="{textColor} flex flex-row my-1">
							<Microchip class="w-5 h-5 mr-2" />
							{agent_hardware.gpu}
						</div>
						<div class="{textColor} flex flex-row my-1">
							<MemoryStick class="w-5 h-5 mr-2" />
							{agent_hardware.ram}
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
								class={agent.status.toString() === 'Online' ? 'text-success-500' : 'text-error-500'}
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
							{agent_hardware.mac_address}
						</div>
					</div>
				</div>
				<div>
					<span class="{textColor} text-xl flex flex-row my-4 justify-start font-bold">
						<HardDrive class="w-7 h-7 mr-2" />
						Storage
					</span>
					<div class="{textColor} px-4 flex flex-col text-center max-h-52 overflow-y-auto">
						{#each agent_hardware.storage as drive}
							<div class="my-2">
								<div class="flex flex-row justify-between">
									{drive.name} ({drive.mountPoint})
								</div>
								<ProgressBar
									value={getDiskPourcentage(drive.usedCapacity, drive.maxCapacity)}
									meter={getDiskPourcentage(drive.usedCapacity, drive.maxCapacity) > 90
										? 'bg-error-500'
										: 'bg-primary-500'}
									track={getDiskPourcentage(drive.usedCapacity, drive.maxCapacity) > 90
										? 'bg-error-500/30'
										: 'bg-primary-500/30'}
								/>
								{drive.availableCapacity} free of {drive.maxCapacity}
							</div>
							<hr />
						{/each}
					</div>
				</div>
			</div>
		</div>
	</div>
	<!-- !Left part (Infos) -->

	<div class="h-96 flex w-full text-primary-100">
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
					on:click={() => modalStore.trigger(modal)}>+</button
				>
			</div>
			<hr />
			<div class="flex flex-grow flex-row my-2">
				<div class="min-w-36 h-full mx-2">
					<div>
						<div class="{textColor} flex flex-col items-center">
							<div class="text-sm mt-2">Pending Tasks</div>
							<div class="text-xl font-bold">
								{agent.tasks.filter((task) => task.status.toString() === 'Pending').length}
							</div>
							<div class="text-sm mt-2">In Progress Tasks</div>
							<div class="text-xl font-bold text-secondary-500">
								{agent.tasks.filter((task) => task.status.toString() === 'InProgress').length}
							</div>
							<div class="text-sm mt-2">Failed Tasks</div>
							<div class="text-xl font-bold text-error-500">
								{agent.tasks.filter((task) => task.status.toString() === 'Failed').length}
							</div>
							<div class="text-sm mt-2">Completed Tasks</div>
							<div class="text-xl font-bold text-success-500">
								{agent.tasks.filter((task) => task.status.toString() === 'Completed').length}
							</div>
							<div class="text-sm mt-2">Total Tasks:</div>
							<div class="text-xl font-bold">{agent.tasks.length}</div>
						</div>
					</div>
				</div>
				<span class="divider-vertical h-64 class m-0 p-0"></span>

				<div class="{textColor} w-full mx-2 max-h-72 overflow-y-auto table-container">
					<table class="table table-hover w-full">
						<thead>
							<tr>
								<th>Task</th>
								<th>Status</th>
								<th>Created At</th>
								<th>Completed At</th>
							</tr>
						</thead>
						<tbody>
							{#each agent.tasks as task}
								<tr>
									<td>{task.task_type}</td>
									<td>
										{#if task.status.toString() === 'Pending'}
											<div class="border border-primary-900-50-token py-1 px-1 w-fit rounded">
												{task.status}
											</div>
										{:else if task.status.toString() === 'InProgress'}
											<div class="bg-secondary-500 py-1 px-1 w-fit rounded">{task.status}</div>
										{:else if task.status.toString() === 'Failed'}
											<div class="bg-error-500 py-1 px-1 w-fit rounded">{task.status}</div>
										{:else if task.status.toString() === 'Completed'}
											<div class="bg-success-500 py-1 px-1 w-fit rounded">{task.status}</div>
										{/if}
									</td>
									<td
										>{new Date(Number(task.emitted_at) * 1000).toLocaleString('en-GB', {
											day: '2-digit',
											month: '2-digit',
											year: 'numeric',
											hour: '2-digit',
											minute: '2-digit',
											second: '2-digit'
										})}</td
									>
									<td
										>{task.completed_at
											? new Date(Number(task.completed_at) * 1000).toLocaleString('en-GB', {
													day: '2-digit',
													month: '2-digit',
													year: 'numeric',
													hour: '2-digit',
													minute: '2-digit',
													second: '2-digit'
												})
											: 'N/A'}</td
									>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
</div>
