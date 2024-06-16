<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import type { FollowRequest } from './types';

	let requests: FollowRequest[] | undefined = undefined;

	onMount(() => {
		invoke('get_follow_requests').then((res) => {
			requests = res as FollowRequest[];
		});
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
