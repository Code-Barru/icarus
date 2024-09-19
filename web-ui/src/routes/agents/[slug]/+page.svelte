<script lang="ts">
	import {
		ArrowDownUp,
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
	import { ProgressBar } from '@skeletonlabs/skeleton';

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
</script>

<!-- Left part (Infos) -->
<div class="h-full flex w-80 text-primary-100">
	<div
		class="flex-grow px-2 bg-surface-100-800-token mx-4 my-2 border border-surface-0-800-token rounded-lg"
	>
		<span class="text-2xl flex flex-row my-4 justify-center font-bold">
			<Monitor class="w-8 h-8 mr-2" />
			{agent.hostname}</span
		>
		<hr />
		<div class="flex flex-col flex-grow">
			<div>
				<span class="text-xl flex flex-row my-4 justify-start font-bold">
					<PcCase class="w-7 h-7 mr-2" />
					Hardware
				</span>
				<div class="pl-4 text-sm justify-start">
					<div class="flex flex-row my-">
						<Cpu class="w-5 h-5 mr-2" />
						{agent_hardware.cpu}
					</div>
					<div class="flex flex-row my-1">
						<Microchip class="w-5 h-5 mr-2" />
						{agent_hardware.gpu}
					</div>
					<div class="flex flex-row my-1">
						<MemoryStick class="w-5 h-5 mr-2" />
						{agent_hardware.ram}
					</div>
				</div>
			</div>
			<div>
				<span class="text-xl flex flex-row my-4 justify-start font-bold">
					<ArrowDownUp class="w-7 h-7 mr-2" />
					Network
				</span>
				<div class="pl-4">
					<div class="flex flex-row my-1">
						{#if agent.status === 'Online'}
							<Wifi class="w-5 h-5 mr-2 stroke-success-500" />
						{:else}
							<WifiOff class="w-5 h-5 mr-2 stroke-error-500" />
						{/if}
						<p class={agent.status.toString() === 'Online' ? 'text-success-500' : 'text-error-500'}>
							{agent.status}
						</p>
					</div>
					<div class="flex flex-row my-1">
						<Router class="w-5 h-5 mr-2" />
						{agent.ip}
					</div>
					<div class="flex flex-row my-1">
						<EthernetPort class="w-5 h-5 mr-2" />
						{agent_hardware.mac_address}
					</div>
				</div>
			</div>
			<div>
				<span class="text-xl flex flex-row my-4 justify-start font-bold">
					<HardDrive class="w-7 h-7 mr-2" />
					Storage
				</span>
				<div class="px-4 flex flex-col text-center max-h-52 overflow-y-auto">
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
