<script lang="ts">
	import { page } from '$app/stores';
	import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';
	import { ClipboardList, House, Settings, UserRound } from 'lucide-svelte';
</script>

<AppRail class="w-32 hidden md:block text-primary-200">
	<AppRailAnchor href="/" selected={$page.url.pathname === '/'} name="Home" title="Home">
		<svelte:fragment slot="lead"><House class="text-primary-700-200-token" /></svelte:fragment>
		<span class="text-primary-700-200-token">Home</span>
	</AppRailAnchor>
	<AppRailAnchor href="/agents" selected={$page.url.pathname === '/agents'} name="Agents">
		<svelte:fragment slot="lead"><UserRound class="text-primary-700-200-token" /></svelte:fragment>
		<span class="text-primary-700-200-token">Agents</span>
	</AppRailAnchor>
	<svelte:fragment slot="trail">
		<AppRailAnchor href="/settings" name="Settings" selected={$page.url.pathname === '/settings'}>
			<svelte:fragment slot="lead"><Settings class="text-primary-700-200-token" /></svelte:fragment>
			<span class="text-primary-700-200-token">Settings</span>
		</AppRailAnchor>
	</svelte:fragment>
</AppRail>
