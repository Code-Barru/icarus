<script lang="ts">
	import AgentElement from '$lib/components/agent/agent-element.svelte';
	import { getAgentState } from '$lib/state.svelte.js';
	import type { Agent } from '$lib/types';
	import type { Writable } from 'svelte/store';

	let agents: Writable<Agent[]> = getAgentState();
</script>

{#if $agents.length >= 0}
	<div class="grid grid-cols-1 grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-6">
		{#each $agents as agent}
			<AgentElement {agent} />
		{/each}
	</div>
{:else}
	<div>
		<p>No agents found</p>
	</div>
{/if}
