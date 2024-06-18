<script lang="ts">
	import { getContext } from 'svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import Timeline from '$lib/model/Timeline.svelte';
	import AccountView from '$lib/model/AccountView.svelte';
	import * as api from '$lib/api';
	import ClientContent from '../client-view/ClientContent.svelte';
	import ExpandedStatus from '../model/ExpandedStatus.svelte';

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
	<ExpandedStatus statusContent={$content}/>
{:else if $content?.type == 'client'}
	<ClientContent content={$content} />
{:else}
	<span>Loading...</span>
{/if}
