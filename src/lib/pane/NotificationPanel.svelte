<script lang="ts">
	import Notification from '$lib/model/Notification.svelte';
	import * as api from '$lib/api';
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import { Pulse } from 'svelte-loading-spinners';
	import { LOADER_COLOR } from '..';

	let notifications: api.Notification[] = [];

	onMount(async () => {
		notifications = await api.fetchNotifications();
	});

	setInterval(async () => {
		const newNotifications = await api.fetchNotifications(notifications[0].id);
		notifications = [...newNotifications, ...notifications]
	}, 15_000);
</script>

<div class="flex flex-row items-center p-2 w-full">
	<h2 class="text-xl">Notifications</h2>
	<div class="flex-grow"/>
	<button
		on:click={async () => {
			notifications = [];
			notifications = await api.fetchNotifications();
		}}
		class="flex flex-row items-end gap-2"
	>
		<Icon
			icon="material-symbols:refresh-rounded"
			height="25"
			width="25"
			class="text-accent"
		/>
	</button>
</div>
<div class="mt-2 flex flex-col gap-2 p-2">
	{#if !notifications.length}
		<span class="text-lg flex flex-row items-center gap-4">
			Fetching notifications...
			<Pulse
				color={LOADER_COLOR}
				size={40}
			/>
		</span>
	{/if}
	{#each notifications as notification}
		<Notification {notification} />
	{/each}
</div>
