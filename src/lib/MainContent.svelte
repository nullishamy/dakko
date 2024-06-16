<script lang="ts">
	import { getContext } from 'svelte';
	import { type Context, mainContext, type StatusContent } from './context';
	import Timeline from './Timeline.svelte';
	import AccountView from './AccountView.svelte';
	import StatusComponent from './Status.svelte';
	import type { Status, StatusContext } from './types';
	import { invoke } from '@tauri-apps/api';

	const { content } = getContext<Context>(mainContext);
	const handleStatusOpen = (status: Status) => {
		invoke('get_conversation', {
			entryPoint: status.id
		}).then((res) => {
			content.set({
				type: 'status',
				openedId: status.reblog?.id ?? status.id,
				status: status,
				statusContext: res as StatusContext,
        onReturn: ($content as StatusContent).onReturn
			});
		});
	};
  console.log(JSON.stringify($content, undefined, 2));
  
</script>

{#if $content?.type == 'user'}
	<AccountView account={$content.account} />
{:else if $content?.type == 'timeline'}
	<Timeline timeline={$content.timeline} />
{:else if $content?.type == 'status'}
	<button on:click={$content.onReturn}>Back</button>
	{#each $content.statusContext.ancestors as status}
		<StatusComponent
			status={status}
			highlighted={status.id === $content.openedId}
			onOpen={handleStatusOpen}
		/>
	{/each}
	{#if !$content.status.reblog}
		<StatusComponent status={$content.status} highlighted onOpen={handleStatusOpen} />
	{/if}
	{#each $content.statusContext.descendants as status}
		<StatusComponent
			{status}
			highlighted={status.id === $content.openedId}
			onOpen={handleStatusOpen}
		/>
	{/each}
{:else}
	<span>Loading...</span>
{/if}
