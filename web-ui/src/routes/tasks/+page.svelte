<script lang="ts">
	export let data;
</script>

<div class="text-2xl font-bold">Tasks</div>
{#if data}
	{#each data.tasks as task}
		<div>{task.task_type}</div>
	{/each}
{/if}
