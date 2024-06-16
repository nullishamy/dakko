<script lang="ts">
	import { formatDistanceStrict } from 'date-fns';
	import type { Account, Status } from './types';
	import CompositionArea from './CompositionArea.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import AccountView from './AccountView.svelte';
	import RenderedContent from './RenderedContent.svelte';
	import { getContext } from 'svelte';
	import { type Context, mainContext } from './context';
	import Icon from '@iconify/svelte';
	import { fullyQualifiedAccount } from './utils';

	export let onOpen: (status: Status) => void;
	export let highlighted: boolean = false;

	export let status: Status;
	const reblog = status.reblog;

	const createdAt = new Date(reblog ? reblog.created_at : status.created_at);
	const { content } = getContext<Context>(mainContext);

	let replyOpen = false;
	const toggleReply = () => {
		replyOpen = !replyOpen;
	};

	let openedUser: Account | undefined;
	const onUserSelect = (user: Account) => {
		openedUser = openedUser?.id == user.id ? undefined : user;
	};

	const handleReply = (data: { content: string; cw: string | undefined }) => {
		invoke('post_reply', {
			postId: status.id,
			reply: data
		});
		toggleReply();
	};

	let hasBeenFavourited = status.favourited ?? false;
	const handleFavourite = () => {
		invoke('favourite_status', {
			statusId: status.id
		}).then(() => {
			hasBeenFavourited = true;
		});
	};

	let hasBeenBoosted = status.reblogged ?? false;
	const handleBoost = () => {
		invoke('boost_status', {
			statusId: reblog ? reblog.id : status.id
		}).then(() => {
			hasBeenBoosted = true;
		});
	};

	let isSensitive = reblog?.sensitive || status.sensitive;
	let showSensitive = false;
	const handleShowSensitive = () => {
		showSensitive = !showSensitive;
	};
</script>

<div class={highlighted ? 'bg-surface0 rounded-md relative p-2' : ' relative p-2'}>
	<div class="flex flex-row items-center justify-between">
		<span class="flex flex-row gap-2 items-center flex-wrap min-w-0">
			{#if reblog}
				<img class="ml-4 rounded-md" height="20" width="20" src={status.account.avatar} />
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]"
					><RenderedContent
						htmlContent={status.account.display_name}
						emojis={status.account.emojis}
					/></span
				>
				<button on:click={() => onUserSelect(status.account)} class="text-sm text-nowrap text-blue"
					>{fullyQualifiedAccount(status.account)}</button
				>
				<span class="subtext0">boosted</span>
			{:else}
				<img height="30" width="30" class="rounded-md" src={status.account.avatar} />
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]"
					><RenderedContent
						htmlContent={status.account.display_name}
						emojis={status.account.emojis}
					/></span
				>
				<button on:click={() => onUserSelect(status.account)} class="text-sm text-nowrap text-blue"
					>{fullyQualifiedAccount(status.account)}</button
				>
			{/if}
		</span>

		<div class="flex flex-row gap-2 whitespace-nowrap">
			<span>{formatDistanceStrict(createdAt, new Date(), { addSuffix: true })}</span>
			<span>Public</span>
		</div>
	</div>

	{#if openedUser}
		<div class="bg-mantle rounded-md z-10 min-w-full border border-pink my-2">
			<AccountView
				account={openedUser}
				isCondensed={true}
				onClose={() => (openedUser = undefined)}
				onOpen={(account) => {
					content.set({
						type: 'user',
						account
					});
				}}
			/>
		</div>
	{/if}

	{#if reblog}
		<span class="flex flex-row gap-2 items-center">
			<img class="rounded-md" height="30" width="30" src={reblog.account.avatar} />
			<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]"
				><RenderedContent
					htmlContent={reblog.account.display_name}
					emojis={reblog.account.emojis}
				/></span
			>
			<button on:click={() => onUserSelect(reblog.account)} class="text-sm text-blue"
				>{fullyQualifiedAccount(reblog.account)}</button
			>
		</span>

		{#if reblog.sensitive && showSensitive}
			<div class="flex flex-col ml-10">
				<em><RenderedContent htmlContent={reblog.spoiler_text} emojis={reblog.emojis} /></em>
				<hr class="bg-pink h-0.5 rounded-lg" />
				<button class="text-blue" on:click={handleShowSensitive}>Hide content</button>
			</div>
			<button class="text-left ml-10" on:click={() => onOpen(reblog)}>
				<RenderedContent htmlContent={reblog.content} emojis={reblog.emojis} />
			</button>
		{:else if reblog.sensitive && !showSensitive}
			<div class="flex flex-col ml-10">
				<em><RenderedContent htmlContent={reblog.spoiler_text} emojis={reblog.emojis} /></em>
				<hr class="bg-pink h-0.5 rounded-lg" />
				<button class="text-blue" on:click={handleShowSensitive}>Show content</button>
			</div>
		{:else}
			<button class="text-left ml-10" on:click={() => onOpen(reblog)}>
				<RenderedContent htmlContent={reblog.content} emojis={reblog.emojis} />
			</button>
		{/if}
	{:else if status.sensitive && showSensitive}
		<div class="flex flex-col ml-10">
			<em><RenderedContent htmlContent={status.spoiler_text} emojis={status.emojis} /></em>
			<hr class="bg-pink h-0.5 rounded-lg" />
			<button class="text-blue" on:click={handleShowSensitive}>Hide content</button>
		</div>
		<button class="text-left ml-10" on:click={() => onOpen(status)}>
			<RenderedContent htmlContent={status.content} emojis={status.emojis} />
		</button>
	{:else if status.sensitive && !showSensitive}
		<div class="flex flex-col ml-10">
			<em><RenderedContent htmlContent={status.spoiler_text} emojis={status.emojis} /></em>
			<hr class="bg-pink h-0.5 rounded-lg" />
			<button class="text-blue" on:click={handleShowSensitive}>Show content</button>
		</div>
	{:else}
		<button class="text-left ml-10" on:click={() => onOpen(status)}>
			<RenderedContent htmlContent={status.content} emojis={status.emojis} />
		</button>
	{/if}

	{#if ((isSensitive && showSensitive) || !isSensitive) && status.media_attachments.length}
		<div class="flex flex-row overflow-x-scroll gap-4 p-1 items-center justify-items-center border border-pink ml-10 mt-2 rounded-md">
			{#each status.media_attachments as attachment}
				<img
          class="w-full"
					src={attachment.url}
					alt={attachment.description ?? 'Unknown'}
				/>
			{/each}
		</div>
	{/if}

	<div class="grid grid-flow-col items-center py-2 rounded-md gap-8 my-2 ml-10">
		<button on:click={toggleReply} class="px-1 border-pink rounded-lg">
			<Icon
				icon="material-symbols:reply"
				class="text-subtext0 hover:text-pink"
				width="25"
				height="25"
			/>
		</button>
		<button on:click={handleBoost} class="px-1 border-pink rounded-lg">
			<Icon
				icon="material-symbols:rocket-launch"
				class={hasBeenBoosted ? 'text-pink' : 'hover:text-pink text-subtext0'}
				width="25"
				height="25"
			/>
		</button>
		<button on:click={handleFavourite} class="px-1 border-pink rounded-lg">
			<Icon
				icon="material-symbols:kid-star"
				class={hasBeenFavourited ? 'text-pink' : 'hover:text-pink text-subtext0'}
				width="25"
				height="25"
			/>
		</button>
		<button class="px-1 border-pink rounded-lg">
			<Icon
				icon="material-symbols:add-reaction"
				class="hover:text-pink text-subtext0"
				width="25"
				height="25"
			/>
		</button>
		<button class="px-1 border-pink rounded-lg">
			<Icon
				icon="material-symbols:more-horiz"
				class="hover:text-pink text-subtext0"
				width="25"
				height="25"
			/>
		</button>
	</div>

	{#if replyOpen}
		<CompositionArea onPost={handleReply} />
	{/if}
</div>
