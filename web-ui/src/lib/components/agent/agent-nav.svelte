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
		class="bg-surface-200-700-token w-full"
	>
		<TabAnchor href="/agents/{data.slug}" selected={$page.url.pathname === `/agents/${data.slug}`}>
			<span>Infos</span>
		</TabAnchor>
		<TabAnchor
			href="/agents/{data.slug}/tasks"
			selected={$page.url.pathname === `/agents/${data.slug}/tasks`}
		>
			<span>Tasks</span>
		</TabAnchor>
	</TabGroup>
</section>
