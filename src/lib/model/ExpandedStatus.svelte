<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import { type MainContext, mainContext, type StatusContent } from '$lib/context';
	import Status from '$lib/model/Status.svelte';
	import * as api from '$lib/api';

	const { content } = getContext<MainContext>(mainContext);
	export let statusContent: StatusContent;
	const handleStatusOpen = async (status: api.Status) => {
		const ctx = await api.fetchStatusContext(status.id);
		content.set({
			type: 'status',
			openedId: status.reblog?.id ?? status.id,
			status: status,
			statusContext: ctx,
			onReturn: ($content as StatusContent).onReturn
		});
	};

	let selected: Element;
  onMount(() => {
		selected?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'nearest' });
  })
</script>

<button on:click={statusContent.onReturn}>Back</button>
<div class="flex flex-col gap-4">
	{#each statusContent.statusContext.ancestors as status}
		<Status
			{status}
			highlighted={status.id === statusContent.openedId}
			onOpen={handleStatusOpen}
		/>
		{#if status.id === statusContent.openedId}
			<div
				class="w-full h-1"
				bind:this={selected}
			/>
		{/if}
	{/each}
	{#if !statusContent.status.reblog}
		<Status
			status={statusContent.status}
			highlighted
			onOpen={handleStatusOpen}
		/>
	{/if}
	{#each statusContent.statusContext.descendants as status}
		<Status
			{status}
			highlighted={status.id === statusContent.openedId}
			onOpen={handleStatusOpen}
		/>
		{#if status.id === statusContent.openedId}
			<div
				class="w-full h-1"
				bind:this={selected}
			/>
		{/if}
	{/each}
</div>
