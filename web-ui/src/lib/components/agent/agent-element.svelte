<script lang="ts">
	import { type Agent } from '$lib/types';
	import { ArrowDownUp, Cable, ClipboardList, Cpu, Fingerprint, Monitor } from 'lucide-svelte';

	export let agent: Agent;
	console.log(agent);
</script>

<a
	href="/agents/{agent.uuid}"
	class="group w-64 h-64 mx-auto bg-surface-100-800-token border border-surface-0-800-token rounded-lg shadow-lg transition transform hover:scale-105"
>
	<span
		class="text-slate-500 justify-center text-lg font-bold group-hover:text-slate-200 transition flex flex-row py-2 px-4"
	>
		<Monitor size="24" class="mr-2" />
		{agent.hostname}
	</span>
	<hr />
	<div class="flex flex-col justify-center h-48">
		<div
			class="{agent.status.toString() === 'Online'
				? 'text-slate-500 group-hover:text-primary-200'
				: 'text-slate-700'} justify-start text-sm transition flex flex-row py-2 px-4"
		>
			<Cable size="20" class="mr-1" />
			{agent.status}
		</div>
		<div
			class="text-slate-500 justify-start text-sm group-hover:text-primary-200 transition flex flex-row py-2 px-4"
		>
			<Fingerprint size="20" class="mr-1" />
			{agent.uuid.slice(0, 18)}
		</div>
		<div
			class="text-slate-500 justify-start text-sm group-hover:text-primary-200 transition flex flex-row py-2 px-4"
		>
			<Cpu size="20" class="mr-1" />
			{agent.platform}
		</div>
		<div
			class="text-slate-500 justify-start text-sm group-hover:text-primary-200 transition flex flex-row py-2 px-4"
		>
			<ArrowDownUp size="20" class="mr-1" />
			{agent.ip}
		</div>
		<div
			class="text-slate-500 justify-start text-sm group-hover:text-primary-200 transition flex flex-row py-2 px-4"
		>
			<ClipboardList size="20" class="mr-1" />
			{agent.tasks.length ? agent.tasks.length : 0}
			{agent.tasks.length === 1 ? 'task' : 'tasks'}
		</div>
	</div>
</a>
