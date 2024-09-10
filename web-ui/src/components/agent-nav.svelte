<script>
	import { TabGroup, TabAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	export let data;
</script>

<section>
	<TabGroup
		justify="justify-center"
		active="variant-filled-primary"
		hover="hover:variant-soft-primary"
		flex="flex-1 lg:flex-none"
		rounded=""
		border=""
		class="bg-surface-100-800-token w-full"
	>
		<TabAnchor href="/agents/{data.slug}" selected={$page.url.pathname === `/agents/${data.slug}`}>
			<!-- <svelte:fragment slot="lead"><Monitor /></svelte:fragment> -->
			<span>Infos</span>
		</TabAnchor>
		<TabAnchor
			href="/agents/{data.slug}/tasks"
			selected={$page.url.pathname === `/agents/${data.slug}/tasks`}
		>
			<!-- <svelte:fragment slot="lead"><ClipboardList /></svelte:fragment> -->
			<span>Tasks</span>
		</TabAnchor>
		<TabAnchor
			href="/agents/{data.slug}/payloads"
			selected={$page.url.pathname === `/agents/${data.slug}/payloads`}
		>
			<!-- <svelte:fragment slot="lead"><Binary /></svelte:fragment> -->
			<span>Payloads</span>
		</TabAnchor>
	</TabGroup>
</section>
