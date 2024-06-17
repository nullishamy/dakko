<script lang="ts">
	import Notification from '$lib/model/Notification.svelte';
	import * as api from '$lib/api';
	import { onMount } from 'svelte';

	let notifications: api.Notification[] = [];

	onMount(async () => {
		notifications = await api.fetchNotifications();
	});

	setInterval(async () => {
		const lastId = notifications[0]?.id;
		if (lastId) {
			const newNotifications = await api.fetchNotifications(lastId);
			console.log('Fetching notifications from', lastId, newNotifications);
			notifications = [...newNotifications, ...notifications];
		} else {
			notifications = await api.fetchNotifications();
		}
	}, 15_000);
</script>

<h2 class="bg-mantle p-1 text-xl">Notifications</h2>
<div class="mt-2 flex flex-col gap-2 p-2">
	{#if !notifications.length}
		<span>Loading...</span>
	{/if}
	{#each notifications as notification}
		<Notification {notification} />
	{/each}
</div>
