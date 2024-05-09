<script lang="ts">
	import { formatDistanceStrict } from 'date-fns';
	import type { Status } from './types';
	import CompositionArea from './CompositionArea.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	export let onOpen: (status: Status) => void;
	export let highlighted: boolean = false;

	export let status: Status;
	const reblog = status.reblog;

	const createdAt = new Date(reblog ? reblog.created_at : status.created_at);

	let replyOpen = false;
	const toggleReply = () => {
		replyOpen = !replyOpen;
	};

	const handleReply = (data: { content: string, cw: string | undefined }) => {
		invoke('post_reply', {
			postId: status.id,
			reply: data
		})
		toggleReply()
	}

	let hasBeenFavourited = status.favourited ?? false
	const handleFavourite = () => {
		invoke('favourite_status', {
			statusId: status.id
		}).then(() => {
			hasBeenFavourited = true
		})
	}
</script>

<div class={highlighted ? 'bg-surface0 p-2 rounded-md' : ' p-2'}>
	<div class="flex flex-row items-center justify-between">
		<button
			on:click={() => onOpen(status)}
			class="flex flex-row gap-2 items-center flex-wrap min-w-0"
		>
			{#if reblog}
				<img class="ml-4 rounded-md" height="20" width="20" src={status.account.avatar} />
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]"
					>{status.account.display_name}</span
				>
				<span class="text-sm text-nowrap">@{status.account.username}</span>
				<span class="subtext0">boosted</span>
			{:else}
				<img height="30" width="30" class="rounded-md" src={status.account.avatar} />
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]"
					>{status.account.display_name}</span
				>
				<span class="text-sm text-nowrap">@{status.account.username}</span>
			{/if}
		</button>

		<div class="flex flex-row gap-2 whitespace-nowrap">
			<span>{formatDistanceStrict(createdAt, new Date(), { addSuffix: true })}</span>
			<span>Public</span>
		</div>
	</div>

	{#if reblog}
		<button on:click={() => onOpen(status)} class="flex flex-row gap-2 items-center">
			<img class="rounded-md" height="30" width="30" src={reblog.account.avatar} />
			<span>{reblog.account.display_name}</span>
			<span class="text-sm">@{reblog.account.username}</span>
		</button>
		<button class="text-left" on:click={() => onOpen(status)}>
			{@html reblog.content}
		</button>
	{:else}
		<button class="text-left" on:click={() => onOpen(status)}>
			{@html status.content}
		</button>
	{/if}

	{#each status.media_attachments as attachment}
		<img src={attachment.url} alt={attachment.description ?? 'Unknown'} />
	{/each}

	<div class="flex flex-row py-2 rounded-md gap-8 my-2">
		<button on:click={toggleReply} class="border border-[0.5] px-1 border-pink rounded-lg">
			Reply
		</button>
		<button class="border border-[0.5] px-1 border-pink rounded-lg">Boost</button>
		<button on:click={handleFavourite} class="border border-[0.5] px-1 border-pink rounded-lg">
			{hasBeenFavourited ? 'Favourited' : 'Favourite'}
		</button>
		<button class="border border-[0.5] px-1 border-pink rounded-lg">React</button>
	</div>

	{#if replyOpen}
		<CompositionArea onPost={handleReply}/>
	{/if}
</div>
