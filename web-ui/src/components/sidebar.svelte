<script lang="ts">
	import { page } from '$app/stores';
	import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';
	import { Binary, ClipboardList, House, Settings, UserRound } from 'lucide-svelte';
</script>

<AppRail class="w-32">
	<AppRailAnchor href="/" selected={$page.url.pathname === '/'} name="Home" title="Home">
		<svelte:fragment slot="lead"><House /></svelte:fragment>
		<span>Home</span>
	</AppRailAnchor>
	<AppRailAnchor href="/agents" selected={$page.url.pathname === '/agents'} name="Agents">
		<svelte:fragment slot="lead"><UserRound /></svelte:fragment>
		<span>Agents</span>
	</AppRailAnchor>
	<AppRailAnchor href="/tasks" selected={$page.url.pathname === '/tasks'} name="Tasks">
		<svelte:fragment slot="lead"><ClipboardList /></svelte:fragment>
		<span>Tasks</span>
	</AppRailAnchor>
	<AppRailAnchor href="/payloads" selected={$page.url.pathname === '/payloads'} name="Payloads">
		<svelte:fragment slot="lead"><Binary /></svelte:fragment>
		<span>Payloads</span>
	</AppRailAnchor>

	<svelte:fragment slot="trail">
		<AppRailAnchor href="/settings" name="Settings" selected={$page.url.pathname === '/settings'}>
			<svelte:fragment slot="lead"><Settings /></svelte:fragment>
			<span>Settings</span>
		</AppRailAnchor>
	</svelte:fragment>
</AppRail>
