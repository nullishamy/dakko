<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import StatusComponent from './Status.svelte';
	import IntersectionObserver from 'svelte-intersection-observer';
	import { ValidTimeline, type Status, type StatusContext } from './types';
	import { capitalise } from './utils';
	import Layout from '../routes/+layout.svelte';

	export let timeline: ValidTimeline;

	const fetchStatuses = (startAt?: string, append?: true, prepend?: true) => {
		invoke(`get_${timeline}_timeline`, { startAt }).then((res) => {
			if (append) {
				statuses = [...statuses, ...(res as Status[])];
			} else {
				statuses = res as Status[];
			}
		});
	};

	let openedStatus: Status | undefined;
	let statusContext: StatusContext | undefined;
	const handleStatusOpen = (status: Status) => {
		
		invoke('get_conversation', {
			entryPoint: status.id
		}).then((res) => {
			openedStatus = status;
			statusContext = res as StatusContext;
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

<div class="flex flex-row bg-mantle p-1 py-2 items-center justify-items-center gap-2">
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
			fetchStatuses();
		}}>Refresh</button
	>
</div>

<div class="mt-2 flex flex-col gap-2 m-1">
	{#if openedStatus && statusContext}
		<button
			on:click={() => {
				openedStatus = undefined;
				statusContext = undefined;
			}}>Back</button
		>
		{#each statusContext.ancestors as status}
			<StatusComponent
				{status}
				highlighted={status.id === openedStatus.reblog?.id}
				onOpen={handleStatusOpen}
			/>
		{/each}
		{#if !openedStatus.reblog}
			<StatusComponent status={openedStatus} highlighted onOpen={handleStatusOpen} />
		{/if}
		{#each statusContext.descendants as status}
			<StatusComponent
				{status}
				highlighted={status.id === openedStatus.reblog?.id}
				onOpen={handleStatusOpen}
			/>
		{/each}
	{:else}
		{#each statuses as status}
			<StatusComponent {status} onOpen={handleStatusOpen} />
		{/each}
	{/if}
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
