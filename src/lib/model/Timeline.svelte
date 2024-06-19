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

	export let timeline: api.InstanceTimeline;

	const { content } = getContext<MainContext>(mainContext);
	const { filters } = getContext<SettingsContext>(settingsContext);

	const replyMap = new Map<string, api.Account>();
	const knownMarkers = new Set<string>();

	export let scrollToPostId: string | undefined;

	let scrollTo: Element;
	let showLoader = false;

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

	let openedStatus: api.Status | undefined;
	const handleStatusOpen = async (status: api.Status) => {
		await openStatus(status, content, () => {
			content.set({
				type: 'timeline',
				timeline,
				cachedStatuses: statuses,
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
				showError(content, err, "when fetching timeline")
				return
			}

			if (!$firstPostInHome) {
				firstPostInHome.set(statuses[0].id);
			}
		}

		scrollTo?.scrollIntoView({ behavior: 'smooth', block: 'center', inline: 'center' });
	});

	setInterval(async () => {
		if ($firstPostInHome) {
			try {
				statusesToCatchup = await fetchCatchupAmount($firstPostInHome);
			} catch (err) {
				showError(content, err, "when fetching timeline catchup")
			}
		}
	}, 10_000);

	const timelines: api.InstanceTimeline[] = [
		api.InstanceTimeline.HOME,
		api.InstanceTimeline.PUBLIC,
		api.InstanceTimeline.BUBBLE,
		api.InstanceTimeline.KNOWN
	];

	let element: HTMLElement;
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
			await fetchStatuses();
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

<div class="mt-2 flex flex-col gap-8 m-1">
	{#each statuses as status, i}
		{@const [filterResult, triggeredFilter] = $filters.filterStatus(status)}
		{#if filterResult == 'show'}
			<Status
				{status}
				replyTo={replyMap.get(status.id)}
				highlighted={status.id === scrollToPostId}
				onOpen={handleStatusOpen}
			/>
		{:else if filterResult === 'warning' && triggeredFilter}
			<FilterWarning
				filter={triggeredFilter}
				{status}
				fromTimeline={timeline}
				cachedStatuses={statuses}
			/>
		{/if}
		{#if status.id == scrollToPostId}
			<div
				bind:this={scrollTo}
				class="w-full h-1"
			/>
		{/if}
		{#if i == statuses.length - 10}
			<div
				class="h-2 w-12"
				bind:this={element}
			></div>
		{/if}
	{/each}

	{#if showLoader || !statuses.length}
		<span class="text-lg flex flex-row items-center gap-2 place-self-center">
			{showLoader ? 'Slow down! Loading more posts..' : 'Fetching your timeline..'}	
			<Pulse
				color={LOADER_COLOR}
				size={30}
			/>
		</span>
	{/if}

	{#if !openedStatus}
		<IntersectionObserver
			{element}
			on:intersect={() => {
				showLoader = true;
				const lastId = statuses[statuses.length - 1]?.id;
				if (knownMarkers.has(lastId)) {
					console.log('Skipping known last id', lastId);
					return;
				}
				knownMarkers.add(lastId);
				if (lastId) {
					fetchStatuses(lastId, true, 25).then(() => {
						showLoader = false;
					});
					console.log('Fetching from', lastId);
				}
			}}
		></IntersectionObserver>
	{/if}
</div>
