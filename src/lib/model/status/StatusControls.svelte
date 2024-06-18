<script lang="ts">
	import * as api from '$lib/api';
	import RenderedContent from '$lib/generic/RenderedContent.svelte';
	import { fullyQualifiedAccount } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { formatDistanceStrict } from 'date-fns';

	export let status: api.Status;
	export let onStatusExpand: (status: api.Status) => void;

	const { reblog } = status;
	const reblogOrStatus = reblog ?? status;
	const createdAt = new Date(reblogOrStatus.created_at);
</script>

<div class="flex flex-row gap-3 items-center whitespace-nowrap font-normal">
	<time datetime={createdAt.toISOString()}>
		{formatDistanceStrict(createdAt, new Date(), { addSuffix: true })}
	</time>

	{#if status.visibility === api.StatusVisibility.DIRECT}
		<span title="Direct">
			<Icon
				icon="mdi:envelope"
				width="20"
				height="20"
			/>
		</span>
	{:else if status.visibility === api.StatusVisibility.PRIVATE}
		<span title="Followers-only">
			<Icon
				icon="mdi:lock"
				width="20"
				height="20"
			/>
		</span>
	{:else if status.visibility === api.StatusVisibility.UNLISTED}
		<span title="Unlisted">
			<Icon
				icon="mdi:unlocked"
				width="20"
				height="20"
			/>
		</span>
	{:else if status.visibility === api.StatusVisibility.PUBLIC}
		<span title="Public">
			<Icon
				icon="mdi:public"
				width="20"
				height="20"
			/>
		</span>
	{/if}

	<button on:click={() => onStatusExpand(status)}>
		<Icon
			icon="material-symbols:keyboard-double-arrow-down"
			width="20"
		/>
	</button>
</div>
