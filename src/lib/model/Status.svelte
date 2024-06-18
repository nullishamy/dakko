<script lang="ts">
	import * as api from '$lib/api';
	import { mainContext, type MainContext } from '$lib/context';
	import CompositionArea from '$lib/generic/CompositionArea.svelte';
	import { fullyQualifiedAccount } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { getContext } from 'svelte';
	import AccountView from './AccountView.svelte';
	import StatusHeader from './StatusHeader.svelte';
	import StatusContent from './StatusContent.svelte';

	export let onOpen: (status: api.Status) => void;
	export let highlighted: boolean = false;

	export let status: api.Status;
	export let replyTo: api.Account | undefined = undefined;

	const reblog = status.reblog;
	const { content } = getContext<MainContext>(mainContext);

	let replyOpen = false;
	const toggleReply = () => {
		replyOpen = !replyOpen;
	};

	let quoteOpen = false;
	const toggleQuote = () => {
		quoteOpen = !quoteOpen;
	};

	let burgerOpen = false;
	const toggleBurger = () => {
		burgerOpen = !burgerOpen;
	};

	let openedUser: api.Account | undefined;
	const onUserSelect = (user: api.Account) => {
		openedUser = openedUser?.id == user.id ? undefined : user;
	};

	const prefillReply = (val: 'quote' | 'reply') => {
		const header = fullyQualifiedAccount((reblog ?? status).account) + ' ';
		let footer = '';
		if (val === 'quote') {
			footer = `\n\nRE: ${status.url}`;
		}
		return header + footer;
	};

	let hasBeenBookmarked = reblog?.bookmarked ?? status.bookmarked ?? false;
	const handleBookmark = async () => {
		if (hasBeenBookmarked) {
			await api.unbookmarkStatus((reblog ?? status).id);
		} else {
			await api.bookmarkStatus((reblog ?? status).id);
		}
	};

	let replyCount = (reblog ?? status).replies_count;
	const handlePost = async (val: 'quote' | 'post', data: api.StatusContent) => {
		if (val === 'quote') {
			await api.postStatus({
				...data,
				quoting: status.id
			});
			toggleQuote();
			return;
		}
		await api.replyToStatus(status.id, data);
		toggleReply();
	};

	let hasBeenFavourited = status.favourited ?? false;
	let favouriteCount = (reblog ?? status).favourites_count;
	const handleFavourite = async () => {
		await api.favouriteStatus((reblog ?? status).id);
		hasBeenFavourited = true;
		favouriteCount += 1;
	};

	let hasBeenBoosted = status.reblogged ?? false;
	let boostCount = (reblog ?? status).reblogs_count;
	const handleBoost = async () => {
		await api.boostStatus((reblog ?? status).id);
		hasBeenBoosted = true;
		boostCount += 1;
	};
</script>

<div class={highlighted ? 'border-2 border-text rounded-md relative p-2' : ' relative p-2'}>
	<StatusHeader
		{status}
		{onUserSelect}
		onStatusExpand={onOpen}
	/>
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
				{fullyQualifiedAccount(replyTo)}
			</span>
		</p>
	{/if}

	<StatusContent
		{status}
		{onUserSelect}
	/>

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
		<button
			class="px-1 border-accent rounded-lg"
			on:click={toggleQuote}
		>
			<Icon
				icon="material-symbols:format-quote"
				class="hover:text-accent text-subtext0"
				width="25"
				height="25"
			/>
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
				<div class="absolute bottom-6 right-1 bg-mantle border border-accent px-4 py-2 rounded-md">
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
		<div class="p-2 bg-mantle rounded-md">
			<CompositionArea
				onPost={(d) => handlePost('post', d)}
				content={prefillReply('reply')}
				cw={status.sensitive ? `re: ${status.spoiler_text}` : undefined}
			/>
		</div>
	{/if}
	{#if quoteOpen}
		<div class="p-2 bg-mantle rounded-md">
			<CompositionArea
				onPost={(d) => handlePost('quote', d)}
				content={prefillReply('quote')}
				cw={status.sensitive ? `re: ${status.spoiler_text}` : undefined}
			/>
		</div>
	{/if}
</div>
