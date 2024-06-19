<script lang="ts">
	import { getContext } from 'svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import Timeline from '$lib/model/Timeline.svelte';
	import AccountView from '$lib/model/AccountView.svelte';
	import ClientContent from '../client-view/ClientContent.svelte';
	import ExpandedStatus from '../model/ExpandedStatus.svelte';
	import { LOADER_COLOR } from '..';
	import { Pulse } from 'svelte-loading-spinners';

	const { content } = getContext<MainContext>(mainContext);
</script>

{#if $content?.type == 'user'}
	<AccountView account={$content.account} />
{:else if $content?.type == 'timeline'}
	<Timeline
		timeline={$content.timeline}
		statuses={$content.cachedStatuses}
		scrollToPostId={$content.scrollToPostId}
	/>
{:else if $content?.type == 'status'}
	<ExpandedStatus statusContent={$content} />
{:else if $content?.type == 'client'}
	<ClientContent content={$content} />
{:else}
	<div class="flex flex-col h-full items-center">
		<span class="text-xl flex flex-row items-center gap-4">
			Loading content
			<Pulse
				color={LOADER_COLOR}
				size={40}
			/>
		</span>
	</div>
{/if}
