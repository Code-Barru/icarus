<script lang="ts">
	import type { Task, TaskType } from '$lib/types';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';
	export let task: Task;

	const modalStore = getModalStore();

	const shellTask: ModalSettings = {
		title: 'Shell Task',
		component: 'shellTask',
		meta: {
			uuid: task.uuid
		},
		type: 'component'
	};

	function openModal() {
		modalStore.trigger(shellTask);
	}

	function truncate(str: string, n: number) {
		return str.length > n ? str.slice(0, n - 1) + '...' : str;
	}
</script>

<tr class="cursor-pointer" on:click={openModal}>
	{#if task.task_type.toString() === 'Shell'}
		<td>{truncate(task.input, 35)}</td>
	{:else}
		<td>{task.task_type}</td>
	{/if}
	<td>
		{#if task.status.toString() === 'Pending'}
			<div class="border border-primary-900-50-token py-1 px-1 w-fit rounded">
				{task.status}
			</div>
		{:else if task.status.toString() === 'InProgress'}
			<div class="bg-secondary-500 py-1 px-1 w-fit rounded">{task.status}</div>
		{:else if task.status.toString() === 'Failed'}
			<div class="bg-error-500 py-1 px-1 w-fit rounded">{task.status}</div>
		{:else if task.status.toString() === 'Completed'}
			<div class="bg-success-500 py-1 px-1 w-fit rounded">{task.status}</div>
		{/if}
	</td>
	<td class="hidden md:table-cell">
		{new Date(Number(task.emitted_at) * 1000).toLocaleString('en-GB', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		})}</td
	>
	<td class="hidden md:table-cell"
		>{task.completed_at
			? new Date(Number(task.completed_at) * 1000).toLocaleString('en-GB', {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric',
					hour: '2-digit',
					minute: '2-digit',
					second: '2-digit'
				})
			: 'N/A'}</td
	>
</tr>
