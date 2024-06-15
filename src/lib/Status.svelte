<script lang="ts">
	import { formatDistanceStrict } from 'date-fns';
	import type { Account, Status } from './types';
	import CompositionArea from './CompositionArea.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import AccountView from './AccountView.svelte';
	import RenderedContent from './RenderedContent.svelte';

	export let onOpen: (status: Status) => void;
	export let highlighted: boolean = false;

	export let status: Status;
	const reblog = status.reblog;

	const createdAt = new Date(reblog ? reblog.created_at : status.created_at);

	let replyOpen = false;
	const toggleReply = () => {
		replyOpen = !replyOpen;
	};

	let openedUser: Account | undefined
	const onUserSelect = (user: Account) => {
		openedUser = openedUser?.id == user.id ? undefined : user
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

	let hasBeenBoosted = status.reblogged ?? false
	const handleBoost = () => {
		invoke('boost_status', {
			statusId: reblog ? reblog.id : status.id
		}).then(() => {
			hasBeenBoosted = true
		})
	}
</script>

<div class={highlighted ? 'bg-surface0 p-2 rounded-md relative' : ' p-2 relative'}>
	{#if openedUser}
		<div class="absolute top-12 left-0 p-2 bg-mantle rounded-md z-10 min-w-full">
			<AccountView account={openedUser}/>
		</div>
	{/if}
	<div class="flex flex-row items-center justify-between">
		<button
			on:click={() => onUserSelect(status.account)}
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
		<button on:click={() => onUserSelect(reblog.account)} class="flex flex-row gap-2 items-center">
			<img class="rounded-md" height="30" width="30" src={reblog.account.avatar} />
			<span>{reblog.account.display_name}</span>
			<span class="text-sm">@{reblog.account.username}</span>
		</button>
		<button class="text-left" on:click={() => onOpen(status)}>
			<RenderedContent htmlContent={reblog.content} emojis={reblog.emojis} />
		</button>
	{:else}
		<button class="text-left" on:click={() => onOpen(status)}>
			<RenderedContent htmlContent={status.content} emojis={status.emojis} />
		</button>
	{/if}

	{#each status.media_attachments as attachment}
		<img src={attachment.url} alt={attachment.description ?? 'Unknown'} />
	{/each}

	<div class="flex flex-row py-2 rounded-md gap-8 my-2">
		<button on:click={toggleReply} class="border border-[0.5] px-1 border-pink rounded-lg">
			Reply
		</button>
		<button on:click={handleBoost} class="border border-[0.5] px-1 border-pink rounded-lg">
			{hasBeenBoosted ? 'Boosted' : 'Boost'}
		</button>
		<button on:click={handleFavourite} class="border border-[0.5] px-1 border-pink rounded-lg">
			{hasBeenFavourited ? 'Favourited' : 'Favourite'}
		</button>
		<button class="border border-[0.5] px-1 border-pink rounded-lg">React</button>
	</div>

	{#if replyOpen}
		<CompositionArea onPost={handleReply}/>
	{/if}
</div>
