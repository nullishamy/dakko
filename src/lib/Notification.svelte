<script lang="ts">
	import { formatDistanceToNowStrict } from 'date-fns';
	import { NotificationType, type Account, type Notification } from './types';
	export let notification: Notification;

	const { type, account, created_at, status } = notification;
	const createdAt = new Date(created_at);
	const showAccount = [
		NotificationType.POLL_EXPIRED,
		NotificationType.FOLLOW,
		NotificationType.REBLOG,
		NotificationType.FAVOURITE,
	].includes(type);
</script>

<div class="flex flex-row flex-wrap items-center">
	{#if showAccount}
		<span>
			{account?.display_name}
			@{account?.username}
		</span>
		<div class="grow" />
		<span>{formatDistanceToNowStrict(createdAt, { addSuffix: true })}</span>
	{/if}
</div>

{#if type === NotificationType.POLL_EXPIRED}
	<span class="text-sm">poll has ended</span>
{:else if type === NotificationType.FOLLOW}
	<span class="text-sm">followed you</span>
{:else if type === NotificationType.REBLOG}
	<span class="text-sm">boosted your post</span>
	<span class="text-xs">{@html status?.content}</span>
{:else if type === NotificationType.FAVOURITE}
	<span class="text-sm">favourited your post</span>
	<span class="text-xs">{@html status?.content}</span>
{:else}
	<span>{JSON.stringify(notification.type, undefined, 2)}</span>
{/if}

<hr class="bg-pink h-0.5 rounded-lg" />
