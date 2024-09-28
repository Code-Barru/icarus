<script lang="ts">
	import '../app.postcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import Sidebar from '$lib/components/sidebar.svelte';
	import Header from '$lib/components/header.svelte';
	import PageTransition from '../transition.svelte';
	import { io } from 'socket.io-client';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { initializeStores, Modal, Drawer, getDrawerStore } from '@skeletonlabs/skeleton';
	import type { ModalComponent } from '@skeletonlabs/skeleton';
	import CreateTask from '$lib/components/modals/create-task.svelte';
	import NavDrawer from '$lib/components/nav-drawer.svelte';
	import {
		addAgent,
		addTask,
		agentDisconnect,
		setAgentState,
		setTaskState,
		updateAgent,
		updateTask
	} from '$lib/state.svelte';
	import { onMount } from 'svelte';
	import { type Agent, type Task } from '$lib/types';
	const modalRegistry: Record<string, ModalComponent> = {
		createTask: { ref: CreateTask }
	};

	initializeStores();
	const drawerStore = getDrawerStore();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	export let data;

	setAgentState(data.agents);
	setTaskState(data.tasks);

	onMount(() => {
		const url = import.meta.env.VITE_C2_URL.replace('http://', 'ws://');

		const ws = io(url);

		const handleAgentCreate = (agent: Agent) => {
			addAgent(agent);
		};
		const handleAgentUpdate = (agent: Agent) => {
			updateAgent(agent);
		};
		const handleAgentDisconnect = (uuid: string) => {
			agentDisconnect(uuid);
		};
		const handletaskCreate = (task: Task) => {
			addTask(task);
		};
		const handleTaskUpdate = (task: Task) => {
			updateTask(task);
		};

		ws.on('connect', () => {
			console.log('Connected to C2');
		});

		ws.on('disconnect', () => {
			console.log('Disconnected from C2');
		});
		ws.on('agent_create', handleAgentCreate);
		ws.on('agent_update', handleAgentUpdate);
		ws.on('agent_disconnect', handleAgentDisconnect);
		ws.on('agent_reconnect', (uuid: string) => {
			console.log('Agent connected', uuid);
		});
		ws.on('task_create', handletaskCreate);
		ws.on('task_update', handleTaskUpdate);

		ws.connect();
	});
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
	<PageTransition url={data.pathname}>
		<slot />
	</PageTransition>
</AppShell>
