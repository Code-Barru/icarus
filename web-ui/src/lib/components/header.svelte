<script lang="ts">
	import { AlignJustify } from 'lucide-svelte';
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import type { DrawerSettings } from '@skeletonlabs/skeleton';

	const drawerStore = getDrawerStore();

	function handleClick() {
		const drawerSettings: DrawerSettings = {
			id: 'nav-drawer',
			width: 'w-full'
		};
		drawerStore.open(drawerSettings);
	}
</script>

<div class="py-4 bg-surface-100-800-token md:hidden sticky border-b border-surface-800-100-token">
	<div class="px-4 w-fit">
		<button on:click={handleClick}>
			<AlignJustify />
		</button>
	</div>
</div>
