<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import StatusComponent from './Status.svelte';
	import IntersectionObserver from 'svelte-intersection-observer';
	import { ValidTimeline, type Status, type StatusContext } from './types';
	import { capitalise } from './utils';
	import { getContext } from 'svelte';
	import { type Context, mainContext } from './context';
	import Icon from '@iconify/svelte';

	export let timeline: ValidTimeline;

	const { content } = getContext<Context>(mainContext)

	const fetchStatuses = (startAt?: string, append?: true) => {
		invoke(`get_${timeline}_timeline`, { startAt }).then((res) => {
			if (append) {
				statuses = [...statuses, ...(res as Status[])];
			} else {
				statuses = res as Status[];
			}
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
						timeline
					})
				}
			});
		});
	};

	let statuses: Status[] = [];
	fetchStatuses();

	const timelines: ValidTimeline[] = [
		ValidTimeline.HOME,
		ValidTimeline.PUBLIC,
		ValidTimeline.BUBBLE,
		ValidTimeline.KNOWN
	];

	let element: HTMLElement;
</script>

<div class="flex flex-row bg-mantle items-center justify-items-center gap-2 sticky top-0 z-10 px-2 py-1 rounded-md">
	{#each timelines as maybeNewTimeline}
		{#if timeline == maybeNewTimeline}
			<h2 class="text-xl">{capitalise(timeline)}</h2>
			<span class="text-2xl text-pink">/</span>
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
			<span class="text-2xl text-pink">/</span>
		{/if}
	{/each}
	<div class="grow" />
	<button
		on:click={() => {
			statuses = [];
			fetchStatuses();
		}}><Icon icon='material-symbols:refresh-rounded' height="25" width="25" class="text-pink"/></button
	>
</div>

<div class="mt-2 flex flex-col gap-4 m-1 px-2">
	{#each statuses as status}
		<StatusComponent {status} onOpen={handleStatusOpen} />
	{/each}
</div>

{#if !openedStatus}
	<IntersectionObserver
		{element}
		on:intersect={(e) => {
			const lastId = statuses[statuses.length - 1]?.id;
			if (lastId) {
				fetchStatuses(lastId, true);
			}
		}}
	>
		<div class="h-2 w-12" bind:this={element}>Loading...</div>
	</IntersectionObserver>
{/if}
