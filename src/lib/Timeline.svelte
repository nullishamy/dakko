<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import StatusComponent from './Status.svelte';
	import IntersectionObserver from 'svelte-intersection-observer';
	import { ValidTimeline, type Status, type StatusContext } from './types';
	import { capitalise } from './utils';
	import { getContext, onMount } from 'svelte';
	import { type MainContext, mainContext } from './context';
	import Icon from '@iconify/svelte';

	export let timeline: ValidTimeline;

	const { content } = getContext<MainContext>(mainContext);

	const replyMap = new Map<string, Status>();

	export let scrollToPostId: string | undefined;

	let scrollTo: Element;

	const fetchStatuses = (startAt?: string, append?: true, limit = 10) => {
		invoke(`get_${timeline}_timeline`, { startAt, limit }).then((_res) => {
			const res = _res as Status[];
			const replyData = Promise.allSettled(
				res
					.filter((r) => r.in_reply_to_id !== null)
					.map(async (r) => {
						const reply = (await invoke('get_status', {
							id: r.in_reply_to_id
						})) as Status;

						replyMap.set(r.id, reply);
					})
			);

			replyData.then(() => {
				if (append) {
					statuses = [...statuses, ...res];
				} else {
					statuses = res;
				}
			});
		});
	};

	let openedStatus: Status | undefined;
	const handleStatusOpen = (status: Status) => {
		invoke('get_conversation', {
			entryPoint: status.id
		}).then((res) => {
			content.set({
				type: 'status',
				openedId: status.reblog?.id ?? status.id,
				status: status,
				statusContext: res as StatusContext,
				onReturn: () => {
					content.set({
						type: 'timeline',
						timeline,
						cachedStatuses: statuses,
						scrollToPostId: status.id
					});
				}
			});
		});
	};

	export let statuses: Status[] = [];
	onMount(() => {
		if (!statuses.length) {
			fetchStatuses();
		}
		scrollTo?.scrollIntoView({ behavior: 'smooth', block: 'center' });
		// FIXME: https://github.com/h3poteto/megalodon-rs/pull/243
		// invoke('get_markers', {
		// 	timelines: ['home', 'notifications']
		// })
	});

	const timelines: ValidTimeline[] = [
		ValidTimeline.HOME,
		ValidTimeline.PUBLIC,
		ValidTimeline.BUBBLE,
		ValidTimeline.KNOWN
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
		on:click={() => {
			statuses = [];
			fetchStatuses();
		}}
	>
		<Icon
			icon="material-symbols:refresh-rounded"
			height="25"
			width="25"
			class="text-accent"
		/>
	</button>
</div>

<div class="mt-2 flex flex-col gap-4 m-1 px-2">
	{#each statuses as status, i}
		<StatusComponent
			{status}
			replyTo={replyMap.get(status.id)}
			onOpen={handleStatusOpen}
		/>
		{#if status.id == scrollToPostId}
			<div bind:this={scrollTo} class="w-full bg-blue h-1"/>
		{/if}
		{#if i == statuses.length - 5}
			<div
				class="h-2 w-12"
				bind:this={element}
			></div>
		{/if}
	{/each}
</div>

{#if !openedStatus}
	<IntersectionObserver
		{element}
		on:intersect={() => {
			const lastId = statuses[statuses.length - 1]?.id;
			if (lastId) {
				fetchStatuses(lastId, true, 15);
				console.log('Fetching from', lastId);

				// FIXME: https://github.com/h3poteto/megalodon-rs/pull/243
				// invoke('save_markers', {
				// 	lastPostInHome: lastId
				// }).then(res => {
				// 	console.log('save', res)
				// }).catch(err => console.error(err))
			}
		}}
	></IntersectionObserver>
	<div class="h-2 w-12">Loading</div>
{/if}
