<script lang="ts">
	import { getDirectoryState, updateAgentExplorerPath } from '$lib/state.svelte';
	import { File, Folder, Upload } from 'lucide-svelte';
	import type { Directory } from '$lib/types.js';

	const C2_URL = `${window.location.protocol}//${window.location.hostname}:1337`;

	export let path: string;
	export let agent: string;

	let dragging: boolean = false;
	let last_changed_drag: Date;

	let directoryState = getDirectoryState();
	let directory: Directory | undefined;

	$: directory = $directoryState.find((d: Directory) => d.path === path && d.agent === agent);

	function updateExplorerPath(file_name: string) {
		const fullPath = path + '/' + file_name;
		updateAgentExplorerPath(agent, fullPath);
		if (!$directoryState.find((d) => d.agent === agent && d.path === fullPath)) {
			fetch(encodeURI(`${C2_URL}/explorer/${agent}?path=${fullPath}`)).catch((error) => {
				console.error('Error:', error);
			});
		}
	}

	function handleDrop(event: any) {
		event.preventDefault();
		dragging = false;
		const files = event.dataTransfer.files;
		const taskData = {
			agent: agent,
			task_type: 'FileUpload',
			input: path + '/' + files[0].name
		};

		fetch(`${C2_URL}/tasks`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(taskData)
		})
			.then((response) => response.json())
			.then((data) => {
				const uuid = data.uuid;
				const formData = new FormData();
				formData.append('file', files[0]);
				fetch(`${C2_URL}/explorer/${uuid}/upload`, {
					method: 'POST',
					body: formData
				}).catch((error) => {
					console.error('Error:', error);
				});
			})
			.catch((error) => {
				console.error('Error:', error);
			});
	}

	function handleDragOver(event: any) {
		event.preventDefault();
		dragging = true;
		last_changed_drag = new Date();
	}

	function handleDragLeave(event: any) {
		event.preventDefault();
		// if no timeout, the drag icon will blink
		setTimeout(() => {
			if (new Date().getTime() - last_changed_drag.getTime() > 10) {
				dragging = false;
			}
		}, 20);
	}
</script>

{#if directory && directory.files}
	<div
		role="complementary"
		aria-label="File Explorer"
		on:drop={handleDrop}
		on:dragover={handleDragOver}
		on:dragleave={handleDragLeave}
		class="h-96"
	>
		{#if !dragging}
			{#if directory.files.length == 0}
				<div class="flex flex-row items-center justify-center p-6">
					<p class="text-primary-100 text-xl text-bold">Empty Folder</p>
				</div>
			{:else}
				<div class="grid grid-cols-1 grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-6">
					{#each directory.files as file}
						{#if file.is_dir}
							<button
								class="flex flex-row items-center text-primary-100 hover:bg-primary-800 hover:bg-opacity-50 py-2 px-2 rounded"
								on:click={() => updateExplorerPath(file.name)}
							>
								<Folder class="w-8 h-8 mr-2" />
								<span class="text-primary-100">{file.name}</span>
							</button>
						{:else}
							<div
								class="flex flex-row items-center text-primary-100 py-2 px-2 hover:bg-primary-800 hover:bg-opacity-50 rounded"
							>
								<File class="w-8 h-8 mr-2" />
								<span class="text-primary-100"
									>{file.name.length > 20 ? file.name.substring(0, 20) + '...' : file.name}</span
								>
							</div>
						{/if}
					{/each}
				</div>
			{/if}
		{:else}
			<div class="w-full h-96 bg-surface-700 border-2 border-primary-700 rounded rounded-2">
				<div class="flex flex-col items-center justify-center p-6">
					<Upload class="w-12 h-12 my-4" />
					<p class="text-primary-100 text-xl text-bold">Drag and drop Files Here</p>
				</div>
			</div>
		{/if}
	</div>
{:else if directory && directory.files === undefined}
	<div class="flex flex-row items-center justify-center p-6">
		<p class="text-primary-100 text-xl text-bold text-error-500">
			Encountered an error while fetching the files
		</p>
	</div>
{:else}
	<div class="flex flex-row items-center justify-center p-6">
		<p class="text-primary-100 text-xl text-bold">Loading...</p>
	</div>
{/if}
