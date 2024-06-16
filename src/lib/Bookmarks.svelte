<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import type { Status, StatusContext } from './types';
	import { invoke } from '@tauri-apps/api';
	import StatusComponent from './Status.svelte';
	import { type MainContext, mainContext, type StatusContent } from './context';

	let bookmarks: Status[] | undefined = undefined;
	const { content } = getContext<MainContext>(mainContext);

	onMount(() => {
		invoke('get_bookmarks').then((res) => {
			bookmarks = res as Status[];
		});
	});

	const handleStatusOpen = (status: Status) => {
		invoke('get_conversation', {
			entryPoint: status.id
		}).then((res) => {
			content.set({
				type: 'status',
				openedId: status.reblog?.id ?? status.id,
				status: status,
				statusContext: res as StatusContext,
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
			<StatusComponent
				status={bookmark}
				onOpen={handleStatusOpen}
			/>
		{/each}
	{/if}
</div>
