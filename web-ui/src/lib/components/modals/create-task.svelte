<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { TaskType } from '$lib/types';
	const C2_URL = import.meta.env.VITE_C2_URL as string;
	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();

	// Form Data
	const formData = {
		agent: $modalStore[0].meta.agent,
		task_type: '',
		input: ''
	};

	// We've created a custom submit function to pass the response and close the modal.
	function onFormSubmit(): void {
		console.log(formData);
		fetch(`${C2_URL}/tasks`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(formData)
		})
			.then((response) => response.json())
			.then((data) => {
				console.log('Success:', data);
				modalStore.close();
			})
			.catch((error) => {
				console.error('Error:', error);
			});
	}

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';
	const cForm = 'border border-surface-500 p-4 space-y-4 rounded-container-token';
	const cInput =
		'bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500';
</script>

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
		<header class={cHeader}>{$modalStore[0].title ?? 'Create a new Task'}</header>
		<!-- Enable for debugging: -->
		<form class="modal-form {cForm}">
			{#if Array.isArray($modalStore[0].meta.agent)}
				<label class="label">
					<span>Agent UUID</span>
					<select class="input {cInput}" bind:value={formData.agent}>
						{#each $modalStore[0].meta.agent as agent}
							<option value={agent}>{agent}</option>
						{/each}
					</select>
				</label>
			{:else if $modalStore[0].meta.agent}
				<label class="label">
					<span>Agent UUID</span>
					<input class="input {cInput}" type="text" bind:value={formData.agent} disabled />
				</label>
			{:else}
				<label class="label">
					<span>Agent UUID</span>
					<input
						class="input {cInput}"
						type="text"
						bind:value={formData.agent}
						placeholder="Enter agent UUID..."
					/>
				</label>
			{/if}
			<label class="label">
				<span>Task Type</span>
				<select class="input {cInput}" bind:value={formData.task_type}>
					{#each Object.values(TaskType) as taskType}
						<option value={taskType}>{taskType}</option>
					{/each}
				</select>
			</label>
			{#if formData.task_type === TaskType.PowerShellCommand || formData.task_type === TaskType.ShellCommand}
				<label class="label">
					<span>Input</span>
					<input class="input {cInput}" type="text" bind:value={formData.input} />
				</label>
			{/if}
		</form>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>{parent.buttonTextSubmit}</button>
		</footer>
	</div>
{/if}
