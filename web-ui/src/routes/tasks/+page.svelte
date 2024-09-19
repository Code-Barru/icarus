<script lang="ts">
	export let data;
</script>

<div class="text-2xl font-bold">Tasks</div>
{#if data}
	{#each data.tasks as task}
		<a href="/tasks/{task.uuid}">{task.task_type}</a>
	{/each}
{/if}
