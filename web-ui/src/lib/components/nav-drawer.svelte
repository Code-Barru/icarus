<script lang="ts">
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import { ClipboardList, House, Settings, UserRound, X } from 'lucide-svelte';

	const drawerStore = getDrawerStore();

	function handleClick() {
		drawerStore.close();
	}
</script>

<div class="flex flex-grow flex-col w-full h-full px-4 py-3">
	<div class="flex justify-end">
		<button on:click={handleClick}>
			<X />
		</button>
	</div>
	<ul>
		<li>
			<a href="/" class="flex flex-row py-4" on:click={handleClick}><House class="pr-1" /> Home</a>
		</li>
		<li>
			<a href="/agents" class="flex flex-row py-4" on:click={handleClick}
				><UserRound class="pr-1" /> Agents</a
			>
		</li>
		<li>
			<a href="/settings" class="flex flex-row py-4" on:click={handleClick}
				><Settings class="pr-1" /> Settings</a
			>
		</li>
	</ul>
</div>
