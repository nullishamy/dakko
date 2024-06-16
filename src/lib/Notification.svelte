<script lang="ts">
	import { formatDistanceToNowStrict } from 'date-fns';
	import {
		NotificationType,
		ValidTimeline,
		type Account,
		type Notification,
		type Status,
		type StatusContext
	} from './types';
	import RenderedContent from './RenderedContent.svelte';
	import Icon from '@iconify/svelte';
	import { getContext } from 'svelte';
	import { type Context, mainContext } from './context';
	import { invoke } from '@tauri-apps/api';
	import { fullyQualifiedAccount } from './utils';
	export let notification: Notification;

	const { content } = getContext<Context>(mainContext);

	const { type, account, created_at, status } = notification;
	const createdAt = new Date(created_at);
	const showAccount = [
		NotificationType.POLL_EXPIRED,
		NotificationType.FOLLOW,
		NotificationType.REBLOG,
		NotificationType.FAVOURITE
	].includes(type);

	const handleOpenStatus = (status: Status) => {
		content.set({ type: 'none' });
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
						timeline: ValidTimeline.HOME
					});
				}
			});
		});
	};
</script>

<div class="flex flex-row flex-wrap items-center">
	{#if showAccount && account}
		<img src={account?.avatar} width="35" height="35" class="rounded-full mr-2" />
		<span>
			<RenderedContent htmlContent={account?.display_name ?? ''} emojis={account?.emojis ?? []} />
			<span class="text-blue">
				{fullyQualifiedAccount(account)}
			</span>
		</span>
		<div class="grow" />
		<span>{formatDistanceToNowStrict(createdAt, { addSuffix: true })}</span>
		{#if status}
			<button class="text-blue ml-2" on:click={() => handleOpenStatus(status)}>
				<Icon width="20" height="20" icon="material-symbols:link" />
			</button>
		{/if}
	{/if}
</div>

{#if type === NotificationType.POLL_EXPIRED}
	<span class="text-sm">poll has ended</span>
{:else if type === NotificationType.FOLLOW}
	<span class="text-sm">followed you</span>
{:else if type === NotificationType.REBLOG}
	<span class="text-sm">boosted your post</span>
	<span class="text-xs"
		><RenderedContent htmlContent={status?.content ?? ''} emojis={status?.emojis ?? []} /></span
	>
{:else if type === NotificationType.FAVOURITE}
	<span class="text-sm">favourited your post</span>
	<span class="text-xs"
		><RenderedContent htmlContent={status?.content ?? ''} emojis={status?.emojis ?? []} /></span
	>
{:else}
	<span>{JSON.stringify(notification.type, undefined, 2)}</span>
{/if}

<hr class="bg-pink h-0.5 rounded-lg" />
