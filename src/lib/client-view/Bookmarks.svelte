<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import * as api from '$lib/api';
	import Status from '$lib/model/Status.svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import { openStatus } from '../utils';

	let bookmarks: api.Status[] | undefined = undefined;
	const { content } = getContext<MainContext>(mainContext);

	onMount(async () => {
		bookmarks = await api.fetchBookmarks();
	});

	const handleStatusOpen = async (status: api.Status) => {
		await openStatus(status, content, () => {
			content.set({
				type: 'client',
				menu: 'bookmarks'
			});
		});
	};
</script>

<div class="w-full m-2">
	{#if bookmarks === undefined}
		<span>Loading...</span>
	{:else if bookmarks !== undefined && !bookmarks.length}
		<span>No bookmarks found</span>
	{:else}
		{#each bookmarks as bookmark}
			<Status
				status={bookmark}
				onOpen={handleStatusOpen}
			/>
		{/each}
	{/if}
</div>
