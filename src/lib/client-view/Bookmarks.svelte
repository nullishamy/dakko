<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import * as api from '$lib/api';
	import { invoke } from '@tauri-apps/api';
	import Status from '$lib/model/Status.svelte';
	import { type MainContext, mainContext } from '$lib/context';

	let bookmarks: api.Status[] | undefined = undefined;
	const { content } = getContext<MainContext>(mainContext);

	onMount(() => {
		invoke('get_bookmarks').then((res) => {
			bookmarks = res as api.Status[];
		});
	});

	const handleStatusOpen = (status: api.Status) => {
		invoke('get_conversation', {
			entryPoint: status.id
		}).then((res) => {
			content.set({
				type: 'status',
				openedId: status.reblog?.id ?? status.id,
				status: status,
				statusContext: res as api.StatusContext,
				onReturn: () => {
					content.set({
						type: 'client',
						menu: 'bookmarks'
					});
				}
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
