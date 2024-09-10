<script lang="ts">
	export let data;
</script>

<div class="text-2xl font-bold">
	<ul class="text-center">
		<li>{data.agent.hostname}</li>
		<li>{data.agent.ip}</li>
		<li>{data.agent.status}</li>
		<li>{data.agent.platform}</li>
	</ul>
</div>
