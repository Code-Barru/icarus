<script lang="ts">
	import AgentElement from '$lib/components/agent/agent-element.svelte';

	export let data;
	let agents = data.agents;
	agents = agents.sort((a, b) => {
		if (a.status === 'Online' && b.status !== 'Online') return -1;
		if (a.status !== 'Online' && b.status === 'Online') return 1;
		return 0;
	});
</script>

{#if data}
	<div class="grid grid-cols-1 grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-6">
		{#each agents as agent}
			<AgentElement {agent} />
		{/each}
	</div>
{/if}
