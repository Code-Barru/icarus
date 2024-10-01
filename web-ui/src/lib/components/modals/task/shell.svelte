<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getTaskState } from '$lib/state.svelte';
	import { getModalStore, CodeBlock } from '@skeletonlabs/skeleton';

	export let parent: SvelteComponent;

	const taskState = getTaskState();
	const modalStore = getModalStore();
	let uuid = $modalStore[0].meta.uuid;
	$: task = $taskState.find((t) => t.uuid === uuid);
</script>

{#if task}
	<div class="card p-4 w-modal shadow-xl space-y-4">
		<header class="text-2xl font-bold">Shell Task</header>
		<div>
			<div class="text-lg mb-2">Input:</div>
			<CodeBlock language="shell" code={task.input}></CodeBlock>
		</div>
		{#if task.response}
			<div>
				<div class="text-lg mb-2">Output:</div>
				<div class="max-h-72 overflow-y-auto">
					<CodeBlock language="shell" code={task.response}></CodeBlock>
				</div>
			</div>
		{/if}
		<footer class={parent.regionFooter}>
			<button class="btn varian-filled-primary" on:click={modalStore.close}>Close</button>
		</footer>
	</div>
{/if}

<style>
	button:focus {
		outline: none !important;
		box-shadow: none !important;
	}
</style>
