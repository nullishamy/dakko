<script lang="ts">
	import { getContext } from 'svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import CompositionArea from '$lib/generic/CompositionArea.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import * as api from '$lib/api';
	import Icon from '@iconify/svelte';

	const { account, content } = getContext<MainContext>(mainContext);
	let composeOpen = false;

	const toggleCompose = () => {
		composeOpen = !composeOpen;
	};

	const postStatus = (status: { content: string; cw: string | undefined }) => {
		invoke('post_status', {
			status
		}).then(() => {
			toggleCompose();
		});
	};

	const openOwnAccount = () => {
		content.set({
			type: 'user',
			account: $account
		});
	};

	const timelineOpener = (timeline: api.InstanceTimeline) => {
		return () => {
			content.set({
				type: 'timeline',
				timeline,
				cachedStatuses: []
			});
		};
	};

	const openSettings = () => {
		content.set({
			type: 'client',
			menu: 'settings'
		});
	};

	const openFollowRequests = () => {
		content.set({
			type: 'client',
			menu: 'follow_requests'
		});
	};

	const openBookmarks = () => {
		content.set({
			type: 'client',
			menu: 'bookmarks'
		});
	};
</script>

<div class="flex flex-col gap-4 items-center justify-items-center h-full">
	<button
		on:click={openOwnAccount}
		class="flex flex-col items-center mb-4"
	>
		<div class="w-[100px] h-[100px]">
			<img
				src={$account?.avatar}
				alt="Avatar for {$account?.display_name}"
				width="100"
				height="100"
				class="bg-mantle rounded-md"
			/>
		</div>
		<span class="text-sm">{$account?.display_name ?? 'Your Display Name'}</span>
		<span class="text-sm">@{$account?.username ?? 'your-username'}</span>
	</button>

	<hr class="bg-accent h-0.5 w-4/5" />
	<button on:click={timelineOpener(api.InstanceTimeline.HOME)}>Timeline</button>
	<hr class="bg-accent h-0.5 w-4/5" />

	<div class="relative w-4/5">
		<button
			class="flex flex-row items-center gap-4"
			on:click={toggleCompose}
		>
			<Icon
				icon="material-symbols:stylus-note"
				width="25"
				height="25"
			/>
			Compose
		</button>
		{#if composeOpen}
			<div class="absolute bg-mantle drop-shadow-md p-2 z-10">
				<CompositionArea onPost={postStatus} />
			</div>
		{/if}
	</div>

	<button
		class="flex flex-row items-center gap-4 w-4/5"
		on:click={openFollowRequests}
	>
		<Icon
			icon="material-symbols:person-add"
			width="25"
			height="25"
		/>
		Requests
	</button>

	<button
		class="flex flex-row items-center gap-4 w-4/5"
		on:click={openBookmarks}
	>
		<Icon
			icon="material-symbols:book"
			width="25"
			height="25"
		/>
		Bookmarks
	</button>

	<hr class="bg-accent h-0.5 w-4/5" />

	<div class="flex-grow" />

	<button
		class="flex flex-row items-center gap-2"
		on:click={openSettings}
	>
		<Icon
			icon="material-symbols:settings"
			width="25"
			height="25"
		/>
		Settings
	</button>
</div>
