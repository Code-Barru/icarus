<script lang="ts">
	export let data;
</script>

<div class="text-2xl font-bold">Agents</div>
{#if data}
	{#each data.agents as agent}
		<a href="/agents/{agent.uuid}">{agent.hostname}</a><br />
	{/each}
{/if}
