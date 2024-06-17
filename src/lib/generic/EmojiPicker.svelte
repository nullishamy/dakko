<script lang="ts">
	import * as api from '$lib/api';
	import { onMount } from 'svelte';
	import { emojiStore } from './emoji-store';
	import { eachMonthOfInterval } from 'date-fns';

	export let onClose = () => {};
	export let onSelect = (emoji: api.CustomEmoji) => {};

	let emojis: api.CustomEmoji[] | undefined = undefined;

	let query: string;
	let currentResults: api.CustomEmoji[] = [];
	$: currentResults = filterResults(query);

	const filterResults = (query: string): api.CustomEmoji[] => {
		const res = emojis?.filter((e) => e.shortcode.includes(query)) ?? [];
		return res.slice(0, 35);
	};

	onMount(async () => {
		if ($emojiStore) {
			emojis = $emojiStore;
		} else {
			emojis = await api.fetchCustomEmojis();
			emojiStore.set(emojis);
		}
		currentResults = emojis.slice(0, Math.min(emojis.length, 35));
	});
</script>

<div class="bg-crust p-2 rounded-md grid grid-cols-7 gap-2 items-center w-96 break-words">
	<button
		on:click={onClose}
		class="mb-2 p-1 border border-accent rounded-md col-span-full"
	>
		Close
	</button>
	<input
		bind:value={query}
		type="text"
		class="col-span-full bg-crust rounded-md"
		placeholder="Search..."
	/>

	{#if emojis === undefined}
		<span class="col-span-full">Loading...</span>
	{:else}
		{#each currentResults as emoji}
			<button
				title={emoji.shortcode}
				on:click={() => onSelect(emoji)}
			>
				<img
					src={emoji.url}
					width="50"
					height="50"
					class="border border-subtext0 rounded-md p-0.5 place-self-center"
					alt={emoji.shortcode}
				/>
			</button>
		{/each}
	{/if}
</div>
