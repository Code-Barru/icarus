<script lang="ts">
	export let data;
	import { getTaskState } from '$lib/state.svelte.js';

	const tasks = getTaskState();
</script>

<div class="text-2xl font-bold">Tasks</div>
{#if data}
	{#each $tasks as task}
		<a href="/tasks/{task.uuid}">{task.task_type}</a>
	{/each}
{/if}
