<script lang="ts">
	import * as api from '$lib/api';
	import Icon from '@iconify/svelte';
	export let status: api.Status;

	const reblog = status.reblog;

	export let replyOpen = false;
	const toggleReply = () => {
		replyOpen = !replyOpen;
	};

	export let quoteOpen = false;
	const toggleQuote = () => {
		quoteOpen = !quoteOpen;
	};

	let burgerOpen = false;
	const toggleBurger = () => {
		burgerOpen = !burgerOpen;
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

<div class="grid grid-flow-col items-center rounded-md gap-8 mt-4">
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
