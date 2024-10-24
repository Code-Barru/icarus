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
	import ShellTask from '$lib/components/modals/task/shell.svelte';
	import NavDrawer from '$lib/components/nav-drawer.svelte';
	import {
		addAgent,
		addDirectory,
		addExplorerAgent,
		addTask,
		agentDisconnect,
		setAgentState,
		setDirectoryState,
		setExplorerState,
		setTaskState,
		updateAgent,
		updateDirectory,
		updateTask
	} from '$lib/state.svelte';
	import { onMount } from 'svelte';
	import { TaskStatus, TaskType, type Agent, type Directory, type Task } from '$lib/types';
	import Hljs from '$lib/components/hljs.svelte';
	const modalRegistry: Record<string, ModalComponent> = {
		createTask: { ref: CreateTask },
		shellTask: { ref: ShellTask }
	};

	initializeStores();
	const drawerStore = getDrawerStore();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	export let data;

	setAgentState(data.agents);
	setTaskState(data.tasks);
	setDirectoryState(data.directories);
	setExplorerState([]);

	onMount(() => {
		const url = import.meta.env.VITE_C2_CLIENT_URL.replace('http://', 'ws://');

		const ws = io(url);

		for (const agent of data.agents) {
			addExplorerAgent(agent.uuid, '', false);
		}

		const handleAgentCreate = (agent: Agent) => {
			addAgent(agent);
			addExplorerAgent(agent.uuid, '', false);
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
			if (task.task_type === TaskType.FileUpload && task.status === TaskStatus.Completed) {
				let path = task.input.replace(/\/[^\/]*$/, '');
				fetch(
					`${import.meta.env.VITE_C2_CLIENT_URL}/explorer/${task.agent}?path=${path}&force=true`
				)
					.then((data) => {
						console.log('Updated');
					})
					.catch((error) => {
						console.error('Error:', error);
					});
			}
			if (task.task_type === TaskType.Explorer && task.status === TaskStatus.Failed) {
				addDirectory({
					agent: task.agent,
					path: task.input,
					files: undefined
				});
			}
		};

		const handleDirectoryCreate = (directory: Directory) => {
			addDirectory(directory);
		};
		const handleDirectoryUpdate = (directory: Directory) => {
			updateDirectory(directory);
		};

		ws.on('connect', () => {
			console.log('Connected to C2');
		});

		ws.on('disconnect', () => {
			setAgentState([]);
			setTaskState([]);
			setDirectoryState([]);
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
		ws.on('directory_create', handleDirectoryCreate);
		ws.on('directory_update', handleDirectoryUpdate);

		ws.connect();
	});
</script>

<svelte:head>
	<title>Icarus</title>
</svelte:head>

<Hljs />
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
