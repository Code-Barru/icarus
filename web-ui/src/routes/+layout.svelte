<script lang="ts">
	import '../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import Sidebar from '$lib/components/sidebar.svelte';
	import PageTransition from '../transition.svelte';
	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { initializeStores, Modal } from '@skeletonlabs/skeleton';
	import type { ModalComponent } from '@skeletonlabs/skeleton';
	import CreateTask from '$lib/components/modals/create-task.svelte';

	const modalRegistry: Record<string, ModalComponent> = {
		createTask: { ref: CreateTask }
	};

	initializeStores();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	export let data;
</script>

<Modal components={modalRegistry} />
<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar />
	</svelte:fragment>
	<PageTransition url={data.url}>
		<slot />
	</PageTransition>
</AppShell>
