<script lang="ts">
	import { Modal, getModalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings, ModalStore } from '@skeletonlabs/skeleton';

	export let data;
	let tasks = data.props.tasks;

	const modalStore: ModalStore = getModalStore();
	const modal: ModalSettings = {
		type: 'component',
		component: 'createTask',
		meta: {
			agent: data.slug
		}
	};
</script>

<div class="text-center text-2xl font-bold">Tasks</div>

{#if tasks}
	{#if tasks.length === 0}
		<div class="text-center">No tasks</div>
	{/if}
	{#if tasks.length > 0}
		{#each tasks as task}
			<div class="text-center">
				<div>{task.task_type}</div>
				<div>{task.status}</div>
				<div>{task.input}</div>
			</div>
		{/each}
	{/if}
{/if}

<button on:click={() => modalStore.trigger(modal)}>New Task</button>
