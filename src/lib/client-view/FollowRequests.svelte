<script lang="ts">
	import { onMount } from 'svelte';
	import * as api from '$lib/api';

	let requests: api.FollowRequest[] | undefined = undefined;

	onMount(async () => {
		requests = await api.fetchFollowRequests()	
	});
</script>

<div class="flex flex-col items-center justify-items-center">
	{#if requests == undefined}
		<span>Loading</span>
	{:else if requests !== undefined && !requests.length}
		<span class="text-2xl font-bold">All caught up, no requests</span>
	{:else}
		{#each requests as request}
			<span>{JSON.stringify(request, undefined, 2)}</span>
		{/each}
	{/if}
</div>
