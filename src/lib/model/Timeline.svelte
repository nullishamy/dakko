<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Status from './Status.svelte';
	import IntersectionObserver from 'svelte-intersection-observer';
	import * as api from '$lib/api';
	import { capitalise, openStatus, showError } from '$lib/utils';
	import { getContext, onMount } from 'svelte';
	import {
		type MainContext,
		mainContext,
		type SettingsContext,
		settingsContext
	} from '$lib/context';
	import Icon from '@iconify/svelte';
	import FilterWarning from './FilterWarning.svelte';
	import { firstPostInHome } from './timeline-store';
	import { Pulse } from 'svelte-loading-spinners';
	import { LOADER_COLOR } from '..';
	import VirtualList from '../generic/virtual-list/VirtualList.svelte';

	export let timeline: api.InstanceTimeline;

	const { content } = getContext<MainContext>(mainContext);
	const { filters } = getContext<SettingsContext>(settingsContext);

	const replyMap = new Map<string, api.Account>();
	const knownMarkers = new Set<string>();

	export let scrollToPostIndex: number | undefined = undefined;
	export let scrollToPostId: string | undefined = undefined;

	let virtualList: VirtualList<api.Status, 'id'>;

	const fetchStatuses = (startAt?: string, append?: true, limit = 25): Promise<void> => {
		return invoke(`get_${timeline}_timeline`, { startAt, limit }).then((_res) => {
			const res = _res as api.Status[];
			res
				.filter((r) => r.in_reply_to_account_id !== null)
				.forEach((r) => {
					const account = r.mentions.find((a) => a.id == r.in_reply_to_account_id);
					if (account) {
						replyMap.set(r.id, account);
					}
				});

			if (append) {
				statuses = [...statuses, ...res];
			} else {
				statuses = res;
			}
		});
	};

	const fetchCatchupAmount = async (sinceId: string) => {
		const posts = await invoke(`get_${timeline}_catchup`, { sinceId });
		return posts as number;
	};

	const handleStatusOpen = async (status: api.Status) => {
		await openStatus(status, content, () => {
			content.set({
				type: 'timeline',
				timeline,
				cachedStatuses: statuses,
				scrollToPostIndex: statuses.findIndex((s) => s.id === status.id),
				scrollToPostId: status.id
			});
		});
	};

	export let statuses: api.Status[] = [];
	let statusesToCatchup = 0;
	onMount(async () => {
		if (!statuses.length) {
			try {
				await fetchStatuses($firstPostInHome);
			} catch (err) {
				showError(content, err, 'when fetching timeline');
				return;
			}

			if (!$firstPostInHome) {
				firstPostInHome.set(statuses[0].id);
			}
		}
	});

	setInterval(async () => {
		if ($firstPostInHome) {
			try {
				// statusesToCatchup = await fetchCatchupAmount($firstPostInHome);
			} catch (err) {
				showError(content, err, 'when fetching timeline catchup');
			}
		}
	}, 15_000);

	const timelines: api.InstanceTimeline[] = [
		api.InstanceTimeline.HOME,
		api.InstanceTimeline.PUBLIC,
		api.InstanceTimeline.BUBBLE,
		api.InstanceTimeline.KNOWN
	];
</script>

<div
	class="flex flex-row bg-mantle items-center justify-items-center gap-2 sticky top-0 z-10 px-2 py-1 rounded-md"
>
	{#each timelines as maybeNewTimeline}
		{#if timeline == maybeNewTimeline}
			<h2 class="text-xl">{capitalise(timeline)}</h2>
			<span class="text-2xl text-accent">/</span>
		{:else}
			<button
				class="text-xl opacity-65"
				on:click={() => {
					timeline = maybeNewTimeline;
					statuses = [];
					knownMarkers.clear()
					scrollToPostId = undefined
					scrollToPostIndex = undefined
					fetchStatuses();
				}}
			>
				{capitalise(maybeNewTimeline)}
			</button>
			<span class="text-2xl text-accent">/</span>
		{/if}
	{/each}
	<div class="grow" />
	<button
		on:click={async () => {
			statuses = [];
			scrollToPostIndex = undefined
			scrollToPostId = undefined
			await fetchStatuses();
			knownMarkers.clear();
			firstPostInHome.set(statuses[0].id);
		}}
		class="flex flex-row items-end gap-2"
	>
		<Icon
			icon="material-symbols:refresh-rounded"
			height="25"
			width="25"
			class="text-accent"
		/>
		{statusesToCatchup > 0 ? `${statusesToCatchup} new` : 'Caught up'}
	</button>
</div>

<VirtualList
	data={statuses}
	key={'id'}
	start={scrollToPostIndex ?? 0}
	footerItemStyles="place-self-center mt-4"
	scrollWrapperStyles="flex flex-col gap-8"
	rootStyles="hide-scrollbar flex flex-col h-full mt-2"
	bind:this={virtualList}
	on:bottom={() => {
		const lastId = statuses[statuses.length - 1]?.id;
		if (knownMarkers.has(lastId)) {
			console.log('Skipping known last id', lastId);
			return;
		}
		knownMarkers.add(lastId);
		if (lastId) {
			fetchStatuses(lastId, true, 25).then(() => {
				setTimeout(() => virtualList.scrollToOffset(virtualList.getOffset() + 1), 3)
			})
			console.log('Fetching from', lastId);
		}
	}}
	let:data
>
	{@const [filterResult, triggeredFilter] = $filters.filterStatus(data)}
	{#if filterResult == 'show'}
		<Status
			status={data}
			replyTo={replyMap.get(data.id)}
			highlighted={data.id === scrollToPostId}
			onOpen={handleStatusOpen}
		/>
	{:else if filterResult === 'warning' && triggeredFilter}
		<FilterWarning
			filter={triggeredFilter}
			status={data}
			fromTimeline={timeline}
			cachedStatuses={statuses}
		/>
	{/if}

	<span
		slot="footer"
		class="text-lg flex flex-row items-center gap-2"
	>
		{statuses.length ? 'Slow down! Loading more posts..' : 'Fetching your timeline..'}
		<Pulse
			color={LOADER_COLOR}
			size={30}
		/>
	</span>
</VirtualList>
