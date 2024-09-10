<script lang="ts">
	import '../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import Sidebar from '../components/sidebar.svelte';
	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { initializeStores, Modal } from '@skeletonlabs/skeleton';
	import type { ModalComponent } from '@skeletonlabs/skeleton';
	import CreateTask from '../components/modals/create-task.svelte';

	const modalRegistry: Record<string, ModalComponent> = {
		createTask: { ref: CreateTask }
	};

	initializeStores();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Modal components={modalRegistry} />
<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar />
	</svelte:fragment>
	<slot />
</AppShell>
