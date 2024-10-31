<script lang="ts">
	import { Ellipsis, Download } from 'lucide-svelte';
	import { addDownloadState } from '$lib/state.svelte';

	export let path: string;
	export let agent: string;

	let C2_URL = import.meta.env.VITE_C2_CLIENT_URL;
	let isOpen = false;
	let isEntered = false;

	function download() {
		fetch(`${C2_URL}/tasks`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				agent: agent,
				task_type: 'FileDownload',
				input: path
			})
		})
			.then((response) => response.json())
			.then((data) => {
				addDownloadState(data.uuid);
			})
			.catch((error) => {
				console.error('Error:', error);
			});
		isOpen = false;
		isEntered = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function mouseEntered() {
		isEntered = true;
	}

	function mouseLeaved() {
		if (!isEntered) return;
		isEntered = false;
		isOpen = false;
	}
</script>

<div
	role="dialog"
	class="relative inline-block text-left"
	on:mouseenter={mouseEntered}
	on:mouseleave={mouseLeaved}
>
	<button
		on:click={toggleDropdown}
		class="inline-flex justify-center w-full my-auto text-sm font-medium rounded-md focus:outline-none"
		id="dropdown-button"
	>
		<Ellipsis />
	</button>

	{#if isOpen}
		<div class="absolute right-0 z-10 w-48 origin-top-right rounded-md bg-surface-700">
			<div class="py-1" role="menu" aria-orientation="vertical">
				<button
					on:click={download}
					class="block px-4 py-2 w-44 mx-auto text-sm font-bold text-surface-100 hover:text-primary-600 hover:bg-surface-600 rounded-md focus:outline-none"
					role="menuitem"
					><div class="flex flex-row"><Download class="w-6 h-6 mr-2" /> Download</div></button
				>
			</div>
		</div>
	{/if}
</div>
