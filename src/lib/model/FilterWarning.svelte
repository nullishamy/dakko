<script lang="ts">
	import { type MainContext, mainContext } from '$lib/context';
	import * as api from '$lib/api';
	import Status from './Status.svelte';
	import { openStatus } from '../utils';
	import { getContext } from 'svelte';
	import { Filter } from '../filtering';

	export let filter: Filter;
	export let status: api.Status;

	export let fromTimeline: api.InstanceTimeline;
	export let cachedStatuses: api.Status[];

	const { content } = getContext<MainContext>(mainContext);

	const handleStatusOpen = async (status: api.Status) => {
		await openStatus(status, content, () => {
			content.set({
				type: 'timeline',
				timeline: fromTimeline,
				cachedStatuses,
				scrollToPostIndex: cachedStatuses.findIndex((s) => s.id === status.id),
				scrollToPostId: status.id
			});
		});
	};

	let hidden = true;
</script>

<div class="p-2 flex flex-col items-center gap-2 my-4 border border-accent rounded-md">
	<span class="font-bold underline text-red">Post hidden by filter ({filter.name})</span>
	<button
		class="text-blue"
		on:click={() => {
			hidden = !hidden;
		}}
	>
		{hidden ? 'Show anyway' : 'Hide again'}
	</button>
	{#if !hidden}
		<div class="place-self-start w-full">
			<Status
				{status}
				onOpen={handleStatusOpen}
			/>
		</div>
	{/if}
</div>
