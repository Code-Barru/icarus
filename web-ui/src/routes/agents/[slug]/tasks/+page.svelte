<script lang="ts">
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { type ModalSettings, type ModalStore } from '@skeletonlabs/skeleton';

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

<div class="flex">
	<button
		on:click={() => modalStore.trigger(modal)}
		class="py-1 px-3 mx-auto my-2 border border-slate-700 border-2 transition-colors rounded hover:bg-slate-700 hover:border-slate-100"
		>New Task</button
	>
</div>

{#if tasks}
	{#if tasks.length === 0}
		<div class="text-center">No tasks</div>
	{/if}
	{#if tasks.length > 0}
		<div class="table-container">
			<table class="table table-hover w-11/12 mx-auto">
				<thead>
					<tr>
						<th>Task Type</th>
						<th>Status</th>
						<th>Emitted At</th>
					</tr>
				</thead>
				<tbody>
					{#each tasks as task}
						<tr class="divide-x border-slate-700">
							<td><a class="block w-full" href="/tasks/{task.uuid}">{task.task_type}</a></td>
							<td><a class="block w-full" href="/tasks/{task.uuid}">{task.status}</a></td>
							<td
								><a class="block w-full" href="/tasks/{task.uuid}"
									>{new Date(Number(task.emitted_at) * 1000).getUTCMonth()}/{new Date(
										Number(task.emitted_at) * 1000
									).getUTCDate()}/{new Date(Number(task.emitted_at) * 1000).getUTCFullYear()}
									- {new Date(Number(task.emitted_at) * 1000).getUTCHours()}h{new Date(
										Number(task.emitted_at) * 1000
									).getUTCMinutes()}:{new Date(Number(task.emitted_at) * 1000).getUTCSeconds()}</a
								></td
							>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
{/if}
