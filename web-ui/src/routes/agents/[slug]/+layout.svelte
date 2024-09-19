<script lang="ts">
	import AgentNav from '$lib/components/agent/agent-nav.svelte';

	export let data;
</script>

<!-- <AgentNav {data} /> -->
<slot {data} />
