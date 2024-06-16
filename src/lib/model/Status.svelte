<script lang="ts">
	import { formatDistanceStrict } from 'date-fns';
	import * as api from '$lib/api';
	import CompositionArea from '$lib/generic/CompositionArea.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import AccountView from './AccountView.svelte';
	import RenderedContent from '$lib/generic/RenderedContent.svelte';
	import { getContext } from 'svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import Icon from '@iconify/svelte';
	import { fullyQualifiedAccount } from '$lib/utils';

	export let onOpen: (status: api.Status) => void;
	export let highlighted: boolean = false;

	export let status: api.Status;
	export let replyTo: api.Status | undefined = undefined;
	export let container: Element | undefined = undefined;
	const reblog = status.reblog;

	const createdAt = new Date(reblog ? reblog.created_at : status.created_at);
	const { content } = getContext<MainContext>(mainContext);

	let replyOpen = false;
	const toggleReply = () => {
		replyOpen = !replyOpen;
	};

	let burgerOpen = false;
	const toggleBurger = () => {
		burgerOpen = !burgerOpen;
	};

	let openedUser: api.Account | undefined;
	const onUserSelect = (user: api.Account) => {
		openedUser = openedUser?.id == user.id ? undefined : user;
	};

	let hasBeenBookmarked = reblog?.bookmarked ?? status.bookmarked ?? false;
	console.log(reblog);
	console.log(status);
	const handleBookmark = () => {
		console.log('haseen', hasBeenBookmarked);

		if (hasBeenBookmarked) {
			invoke('unbookmark_status', {
				id: (reblog ?? status).id
			}).then(() => {
				hasBeenBookmarked = false;
			});
		} else {
			invoke('bookmark_status', {
				id: (reblog ?? status).id
			}).then(() => {
				hasBeenBookmarked = true;
			});
		}
	};

	let replyCount = (reblog ?? status).replies_count;
	const handleReply = (data: {}) => {
		invoke('post_reply', {
			postId: status.id,
			reply: data
		});
		toggleReply();
	};

	let hasBeenFavourited = status.favourited ?? false;
	let favouriteCount = (reblog ?? status).favourites_count;
	const handleFavourite = () => {
		invoke('favourite_status', {
			statusId: status.id
		}).then(() => {
			hasBeenFavourited = true;
			favouriteCount += 1;
		});
	};

	let hasBeenBoosted = status.reblogged ?? false;
	let boostCount = (reblog ?? status).reblogs_count;
	const handleBoost = () => {
		invoke('boost_status', {
			statusId: reblog ? reblog.id : status.id
		}).then(() => {
			hasBeenBoosted = true;
			boostCount += 1;
		});
	};

	let isSensitive = reblog?.sensitive || status.sensitive;
	let showSensitive = false;
	const handleShowSensitive = () => {
		showSensitive = !showSensitive;
	};
</script>

<div
	class={highlighted ? 'bg-surface0 rounded-md relative p-2' : ' relative p-2'}
	bind:this={container}
>
	<div class="flex flex-row items-center justify-between">
		<span class="flex flex-row gap-2 items-center flex-wrap min-w-0">
			{#if reblog}
				<img
					class="ml-4 rounded-md"
					height="20"
					width="20"
					src={status.account.avatar}
				/>
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]">
					<strong>
						<RenderedContent
							htmlContent={status.account.display_name}
							emojis={status.account.emojis}
						/>
					</strong>
				</span>
				<button
					on:click={() => onUserSelect(status.account)}
					class="text-sm text-nowrap text-blue"
				>
					{fullyQualifiedAccount(status.account)}
				</button>
				<span class="subtext0">boosted</span>
			{:else}
				<img
					height="30"
					width="30"
					class="rounded-md"
					src={status.account.avatar}
				/>
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]">
					<strong>
						<RenderedContent
							htmlContent={status.account.display_name}
							emojis={status.account.emojis}
						/>
					</strong>
				</span>
				<button
					on:click={() => onUserSelect(status.account)}
					class="text-sm text-nowrap text-blue"
				>
					{fullyQualifiedAccount(status.account)}
				</button>
			{/if}
		</span>

		<div class="flex flex-row gap-3 items-center whitespace-nowrap">
			<time datetime={createdAt.toISOString()}>
				{formatDistanceStrict(createdAt, new Date(), { addSuffix: true })}
			</time>
			<button on:click={() => onOpen(status)}>
				<Icon
					icon="material-symbols:keyboard-double-arrow-down"
					width="20"
				/>
			</button>
		</div>
	</div>

	{#if openedUser}
		<div class="bg-mantle rounded-md z-10 min-w-full border border-accent my-2">
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

	{#if replyTo}
		<p class="ml-10 text-subtext0 text-sm mb-2 flex flex-row items-center gap-2">
			<Icon
				icon="material-symbols:reply"
				flip="horizontal"
			/>
			Reply to
			<span class="text-blue gap-0">
				{fullyQualifiedAccount(replyTo.account)}
			</span>
		</p>
	{/if}

	{#if reblog}
		<span class="flex flex-row gap-2 items-center">
			<img
				class="rounded-md"
				height="30"
				width="30"
				src={reblog.account.avatar}
			/>
			<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]">
				<strong>
					<RenderedContent
						htmlContent={reblog.account.display_name}
						emojis={reblog.account.emojis}
					/>
				</strong>
			</span>
			<button
				on:click={() => onUserSelect(reblog.account)}
				class="text-sm text-blue"
			>
				{fullyQualifiedAccount(reblog.account)}
			</button>
		</span>

		{#if reblog.sensitive && showSensitive}
			<div class="flex flex-col ml-10">
				<em>
					<RenderedContent
						htmlContent={reblog.spoiler_text}
						emojis={reblog.emojis}
					/>
				</em>
				<hr class="bg-accent h-0.5 rounded-lg" />
				<button
					class="text-blue"
					on:click={handleShowSensitive}
				>
					Hide content
				</button>
			</div>
			<div class="text-left ml-10">
				<RenderedContent
					htmlContent={reblog.content}
					emojis={reblog.emojis}
				/>
			</div>
		{:else if reblog.sensitive && !showSensitive}
			<div class="flex flex-col ml-10">
				<em>
					<RenderedContent
						htmlContent={reblog.spoiler_text}
						emojis={reblog.emojis}
					/>
				</em>
				<hr class="bg-accent h-0.5 rounded-lg" />
				<button
					class="text-blue"
					on:click={handleShowSensitive}
				>
					Show content
				</button>
			</div>
		{:else}
			<div class="text-left ml-10">
				<RenderedContent
					htmlContent={reblog.content}
					emojis={reblog.emojis}
				/>
			</div>
		{/if}
	{:else if status.sensitive && showSensitive}
		<div class="flex flex-col ml-10">
			<em>
				<RenderedContent
					htmlContent={status.spoiler_text}
					emojis={status.emojis}
				/>
			</em>
			<hr class="bg-accent h-0.5 rounded-lg" />
			<button
				class="text-blue"
				on:click={handleShowSensitive}
			>
				Hide content
			</button>
		</div>
		<div class="text-left ml-10">
			<RenderedContent
				htmlContent={status.content}
				emojis={status.emojis}
			/>
		</div>
	{:else if status.sensitive && !showSensitive}
		<div class="flex flex-col ml-10">
			<em>
				<RenderedContent
					htmlContent={status.spoiler_text}
					emojis={status.emojis}
				/>
			</em>
			<hr class="bg-accent h-0.5 rounded-lg" />
			<button
				class="text-blue"
				on:click={handleShowSensitive}
			>
				Show content
			</button>
		</div>
	{:else}
		<div class="text-left ml-10">
			<RenderedContent
				htmlContent={status.content}
				emojis={status.emojis}
			/>
		</div>
	{/if}

	{#if ((isSensitive && showSensitive) || !isSensitive) && status.media_attachments.length}
		<div
			class="flex flex-row overflow-x-scroll gap-4 p-1 items-center justify-items-center border border-accent ml-10 mt-2 rounded-md"
		>
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
		<button
			on:click={toggleReply}
			class="px-1 border-accent rounded-lg flex flex-row gap-2"
		>
			<Icon
				icon="material-symbols:reply"
				class="text-subtext0 hover:text-accent"
				width="25"
				height="25"
			/>
			<span class="text-lg">{replyCount}</span>
		</button>
		<button
			on:click={handleBoost}
			class="px-1 border-accent rounded-lg flex flex-row gap-2"
		>
			<Icon
				icon="material-symbols:rocket-launch"
				class={hasBeenBoosted ? 'text-accent' : 'hover:text-accent text-subtext0'}
				width="25"
				height="25"
			/>
			<span class="text-lg">{boostCount}</span>
		</button>
		<button
			on:click={handleFavourite}
			class="px-1 border-accent rounded-lg flex flex-row gap-2"
		>
			<Icon
				icon="material-symbols:kid-star"
				class={hasBeenFavourited ? 'text-accent' : 'hover:text-accent text-subtext0'}
				width="25"
				height="25"
			/>
			<span class="text-lg">{favouriteCount}</span>
		</button>
		<button class="px-1 border-accent rounded-lg">
			<Icon
				icon="material-symbols:add-reaction"
				class="hover:text-accent text-subtext0"
				width="25"
				height="25"
			/>
		</button>
		<button
			class="px-1 border-accent rounded-lg relative"
			on:click={toggleBurger}
		>
			<Icon
				icon="material-symbols:more-horiz"
				class="hover:text-accent text-subtext0"
				width="25"
				height="25"
			/>

			{#if burgerOpen}
				<div class="absolute bottom-6 bg-mantle border border-accent px-4 py-2 rounded-md">
					<button
						class="flex flex-row items-center gap-2"
						on:click={handleBookmark}
					>
						<Icon icon="material-symbols:book" />
						{hasBeenBookmarked ? 'Unbookmark' : 'Bookmark'}
					</button>
				</div>
			{/if}
		</button>
	</div>

	{#if replyOpen}
		<CompositionArea
			onPost={handleReply}
			data={{
				cw: status.sensitive ? `re: ${status.spoiler_text}` : undefined,
				content: fullyQualifiedAccount(status.account)
			}}
		/>
	{/if}
</div>
