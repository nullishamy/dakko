<script lang="ts">
	import { formatDistanceToNowStrict } from 'date-fns';
	import * as api from '$lib/api';
	import RenderedContent from '$lib/generic/RenderedContent.svelte';
	import Icon from '@iconify/svelte';
	import { getContext } from 'svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import { invoke } from '@tauri-apps/api';
	import { fullyQualifiedAccount } from '$lib/utils';

	export let notification: api.Notification;

	const { content } = getContext<MainContext>(mainContext);

	const { type, account, created_at, status, target } = notification;
	const createdAt = new Date(created_at);
	const showAccount = [
		api.NotificationType.POLL_EXPIRED,
		api.NotificationType.FOLLOW,
		api.NotificationType.REBLOG,
		api.NotificationType.FAVOURITE,
		api.NotificationType.MENTION,
		api.NotificationType.MOVE
	].includes(type);

	const handleOpenStatus = (status: api.Status) => {
		content.set({ type: 'none' });
		invoke('get_conversation', {
			entryPoint: status.id
		}).then((res) => {
			content.set({
				type: 'status',
				openedId: status.reblog?.id ?? status.id,
				status: status,
				statusContext: res as api.StatusContext,
				onReturn: () => {
					content.set({
						type: 'timeline',
						timeline: api.InstanceTimeline.HOME,
						cachedStatuses: []
					});
				}
			});
		});
	};

	const openAccount = (account: api.Account) => {
		console.log('notification', account);
		content.set({
			type: 'user',
			account
		});
	};
</script>

<div class="flex flex-row flex-wrap items-center">
	{#if showAccount && account}
		<button on:click={() => openAccount(account)}>
			<img
				src={account?.avatar}
				width="35"
				height="35"
				class="rounded-full mr-2"
			/>
		</button>
		<span>
			<RenderedContent
				htmlContent={account?.display_name ?? ''}
				emojis={account?.emojis ?? []}
			/>
			<span class="text-blue">
				{fullyQualifiedAccount(account)}
			</span>
		</span>
		<div class="grow" />
		<span>{formatDistanceToNowStrict(createdAt, { addSuffix: true })}</span>
		{#if status}
			<button
				class="text-blue ml-2"
				on:click={() => handleOpenStatus(status)}
			>
				<Icon
					width="20"
					height="20"
					icon="material-symbols:link"
				/>
			</button>
		{/if}
	{/if}
</div>

{#if type === api.NotificationType.POLL_EXPIRED}
	<span class="text-sm">poll has ended</span>
{:else if type === api.NotificationType.FOLLOW}
	<span class="text-sm">followed you</span>
{:else if type === api.NotificationType.MOVE && target}
	<span class="text-sm">
		migrated to <span class="text-blue">{fullyQualifiedAccount(target)}</span>
	</span>
{:else if type === api.NotificationType.REBLOG && status}
	<span class="text-sm">boosted your post</span>
	<span class="text-xs">
		<RenderedContent
			htmlContent={status.content}
			emojis={status.emojis}
		/>
	</span>
{:else if type === api.NotificationType.FAVOURITE && status}
	<span class="text-sm">favourited your post</span>
	<span class="text-xs">
		<RenderedContent
			htmlContent={status.content}
			emojis={status.emojis}
		/>
	</span>
{:else if type === api.NotificationType.MENTION && status}
	<span class="text-sm">mentioned you</span>
	<span class="text-xs">
		<RenderedContent
			htmlContent={status.content}
			emojis={status.emojis}
		/>
	</span>
{:else}
	<span>{JSON.stringify(notification.type, undefined, 2)}</span>
{/if}

<hr class="bg-accent h-0.5 rounded-lg" />
