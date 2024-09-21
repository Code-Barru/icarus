<script lang="ts">
	import '../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import Sidebar from '$lib/components/sidebar.svelte';
	import Header from '$lib/components/header.svelte';
	import PageTransition from '../transition.svelte';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { initializeStores, Modal, Drawer, getDrawerStore } from '@skeletonlabs/skeleton';
	import type { ModalComponent } from '@skeletonlabs/skeleton';
	import CreateTask from '$lib/components/modals/create-task.svelte';
	import NavDrawer from '$lib/components/nav-drawer.svelte';
	const modalRegistry: Record<string, ModalComponent> = {
		createTask: { ref: CreateTask }
	};

	initializeStores();
	const drawerStore = getDrawerStore();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	export let data;
</script>

<svelte:head>
	<title>Icarus</title>
</svelte:head>

<Modal components={modalRegistry} />
<Drawer>
	{#if $drawerStore.id === 'nav-drawer'}
		<NavDrawer />
	{/if}
</Drawer>
<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar />
	</svelte:fragment>
	<svelte:fragment slot="header"><Header /></svelte:fragment>
	<PageTransition url={data.url}>
		<slot />
	</PageTransition>
</AppShell>
