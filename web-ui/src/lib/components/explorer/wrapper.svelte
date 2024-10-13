<script lang="ts">
	import { getDirectoryState, updateAgentExplorerPath } from '$lib/state.svelte';
	import { File, Folder } from 'lucide-svelte';
	import type { Directory, File as FileType } from '$lib/types.js';

	const C2_URL = `${window.location.protocol}//${window.location.hostname}:1337`;

	export let path: string;
	export let agent: string;

	let directoryState = getDirectoryState();
	let directory: Directory | undefined;

	$: directory = $directoryState.find((d: Directory) => d.path === path && d.agent === agent);

	function updateExplorerPath(file_name: string) {
		if (path.endsWith('/') || path.endsWith('\\')) {
			path = path.slice(0, -1);
		}
		let fullPath = `${path}/${file_name}`;
		console.log('Updating path:', fullPath);
		updateAgentExplorerPath(agent, fullPath);
		if (!$directoryState.find((d) => d.agent === agent && d.path === fullPath)) {
			console.log('Fetching:', `${C2_URL}/explorer/${agent}?path=${fullPath}`);
			fetch(encodeURI(`${C2_URL}/explorer/${agent}?path=${fullPath}`)).catch((error) => {
				console.error('Error:', error);
			});
		} else {
			console.log('Path already exists:', path);
		}
	}
</script>

{#if directory && directory.files}
	{#if directory.files.length == 0}
		<div class="flex flex-row items-center justify-center p-6">
			<p class="text-primary-100 text-xl text-bold">Empty Folder</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 grid-cols-[repeat(auto-fill,minmax(256px,1fr))] gap-4 p-6">
			{#each directory.files as file}
				{#if file.is_dir}
					<button
						class="flex flex-row items-center text-primary-100"
						on:click={() => updateExplorerPath(file.name)}
					>
						<Folder class="w-8 h-8 mr-2" />
						<span class="text-primary-100">{file.name}</span>
					</button>
				{:else}
					<div class="flex flex-row items-center text-primary-100">
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
	<div class="flex flex-row items-center justify-center p-6">
		<p class="text-primary-100 text-xl text-bold">Loading...</p>
	</div>
{/if}
