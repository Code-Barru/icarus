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
		fetch(`${C2_URL}/tasks`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(formData)
		})
			.then((response) => response.json())
			.then((data) => {
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
		'outline-none focus:ring-0 bg-surface-50 border border-surface-300 text-primary-500 font-bold text-sm rounded-lg focus:ring-primary-200-800-token focus:border-primary-200-800-token block w-full p-2';
</script>

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
		<header class={cHeader}>{'Create a new Task'}</header>
		<form class="modal-form {cForm}">
			<label class="label">
				<span class="text-primary-100">Agent UUID</span>
				<input class="input {cInput}" type="text" bind:value={formData.agent} disabled />
			</label>
			<label class="label">
				<span class="text-primary-100">Task Type</span>
				<select class="input {cInput}" bind:value={formData.task_type}>
					{#each Object.values(TaskType) as taskType}
						<option class="hover:bg-secondary-200-800-token" value={taskType}>{taskType}</option>
					{/each}
				</select>
			</label>
			{#if formData.task_type === TaskType.Shell}
				<label class="label">
					<span class="text-primary-100">Input</span>
					<input class="input {cInput}" type="text" bind:value={formData.input} />
				</label>
			{/if}
		</form>
		<!-- prettier-ignore -->
		<footer class="{parent.regionFooter}">
			<button class="btn variant-ghost " on:click={parent.onClose}>Cancel</button>
			<button class="btn variant-filled-primary" on:click={onFormSubmit}>Create</button>
		</footer>
	</div>
{/if}

<style>
	button:focus {
		outline: none !important;
		box-shadow: none !important;
	}
</style>
