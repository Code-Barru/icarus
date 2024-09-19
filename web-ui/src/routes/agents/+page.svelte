<script lang="ts">
	import AgentElement from '$lib/components/agent-element.svelte';

	export let data;
	let agents = data.agents;
	agents = agents.sort((a, b) => {
		if (a.status === 'Online' && b.status !== 'Online') return -1;
		if (a.status !== 'Online' && b.status === 'Online') return 1;
		return 0;
	});
</script>

<div
	class="text-2xl font-bold text-center w-full bg-surface-200-700-token py-1 border-b border-surface-700-200-token"
>
	Agents
</div>
{#if data}
	<div class="grid grid-cols-1 grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-6">
		{#each agents as agent}
			<AgentElement {agent} />
		{/each}
	</div>
{/if}
