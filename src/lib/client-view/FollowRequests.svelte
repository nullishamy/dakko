<script lang="ts">
	import { onMount } from 'svelte';
	import * as api from '$lib/api';
	import { fullyQualifiedAccount } from '../utils';
	import { Pulse } from 'svelte-loading-spinners';
	import { LOADER_COLOR } from '..';

	let requests: api.FollowRequest[] | undefined = undefined;

	onMount(async () => {
		requests = await api.fetchFollowRequests();
	});

	const acceptRequest = async (request: api.Account) => {
		await api.acceptFollowRequest(request.id);
		requests = await api.fetchFollowRequests();
	};
	const denyRequest = async (request: api.Account) => {
		await api.denyFollowRequest(request.id);
		requests = await api.fetchFollowRequests();
	};
</script>

<div class="flex flex-col justify-items-center px-4 gap-2">
	{#if requests == undefined}
		<span class="text-xl flex flex-row items-center gap-2 place-self-center">
			Loading follow requests
			<Pulse color={LOADER_COLOR} size={30}/>
		</span>
	{:else if requests !== undefined && !requests.length}
		<span class="text-2xl place-self-center">All caught up, no requests</span>
	{:else}
		{#each requests as request}
			<div class="flex flex-row gap-2 min-h-16">
				<img
					src={request.avatar}
					alt={`avatar for ${request.username}`}
					class="w-16"
				/>
				<div class="flex flex-col items-start">
					<span class="text-lg">{request.display_name}</span>
					<a href={request.url}>{fullyQualifiedAccount(request)}</a>
				</div>
			</div>
			<div class="flex flex-row items-center gap-4">
				<button
					on:click={() => acceptRequest(request)}
					class="p-1 w-24 bg-mantle rounded-md border border-accent"
				>
					Approve
				</button>
				<button
					on:click={() => denyRequest(request)}
					class="p-1 w-24 bg-red border border-red rounded-md text-base"
				>
					Deny
				</button>
			</div>
			<hr class="h-0.5 w-full rounded-lg bg-accent" />
			<!-- <span>{JSON.stringify(request, undefined, 2)}</span> -->
		{/each}
	{/if}
</div>
