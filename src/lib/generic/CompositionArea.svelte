<script lang="ts">
	import * as api from '$lib/api';
	import Icon from '@iconify/svelte';
	import EmojiPicker from './EmojiPicker.svelte';
	import { capitalise } from '../utils';

	export let onPost: (data: api.StatusContent) => void;

	export let content: string = '';
	export let cw: string | undefined = undefined;
	export let visibility: api.StatusVisibility = api.StatusVisibility.PUBLIC;

	let pickerOpen = false;

	const handlePost = () => {
		onPost({ content, cw, visibility });
	};

	const setVisibility = (vis: api.StatusVisibility) => {
		visibility = vis;
	};
</script>

<div class="min-w-96">
	<textarea
		bind:value={cw}
		class="bg-surface0 w-full min-h-10 text-text p-1"
		placeholder="Content warning (optional)"
	></textarea>

	<textarea
		bind:value={content}
		required
		class="bg-surface0 w-full min-h-36 text-text p-1"
		placeholder="Something creative..."
	></textarea>
	{#if pickerOpen}
		<div class="absolute bottom-auto">
			<EmojiPicker
				onClose={() => {
					pickerOpen = false;
				}}
				onSelect={(emoji) => {
					content += `:${emoji.shortcode}:`;
				}}
			/>
		</div>
	{/if}
</div>

<div class="flex flex-row gap-3 items-center">
	<button
		on:click={() => {
			setVisibility(api.StatusVisibility.DIRECT);
		}}
		class="flex flex-row items-center gap-2 border border-accent p-0.5 rounded-md"
	>
		<Icon
			icon="mdi:envelope"
			width="25"
			height="25"
		/>
	</button>
	<button
		on:click={() => {
			setVisibility(api.StatusVisibility.PRIVATE);
		}}
		class="flex flex-row items-center gap-2 border border-accent p-0.5 rounded-md"
	>
		<Icon
			icon="mdi:lock"
			width="25"
			height="25"
		/>
	</button>
	<button
		on:click={() => {
			setVisibility(api.StatusVisibility.UNLISTED);
		}}
		class="flex flex-row items-center gap-2 border border-accent p-0.5 rounded-md"
	>
		<Icon
			icon="mdi:unlocked"
			width="25"
			height="25"
		/>
	</button>
	<button
		on:click={() => {
			setVisibility(api.StatusVisibility.PUBLIC);
		}}
		class="flex flex-row items-center gap-2 border border-accent p-0.5 rounded-md"
	>
		<Icon
			icon="mdi:public"
			width="25"
			height="25"
		/>
	</button>
	<div class="grow" />
	<span>{capitalise(visibility)}</span>
</div>

<div class="flex flex-row gap-3 items-center">
	<button
		on:click={() => {
			pickerOpen = !pickerOpen;
		}}
		class="flex flex-row items-center gap-2 border border-accent p-0.5 rounded-md"
	>
		<Icon
			icon="material-symbols:add-reaction"
			width="25"
			height="25"
		/>
		Emoji
	</button>

	<div class="grow" />

	<button
		class="bg-surface0 p-2 rounded-lg min-w-36"
		on:click={handlePost}
	>
		Post
	</button>
</div>
