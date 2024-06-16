<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Notification from '$lib/model/Notification.svelte';
	import * as api from '$lib/api';
	import { onMount } from 'svelte';

	let notifications: api.Notification[] = [];

	onMount(() => {
		invoke('get_notifications').then((res) => {
			notifications = res as api.Notification[];
		});
	});

	setInterval(() => {
		const lastId = notifications[0]?.id;
		if (lastId) {
			invoke('get_notifications', {
				since: lastId
			}).then((res) => {
				console.log('Fetching notifications from', lastId, res);
				notifications = [...(res as api.Notification[]), ...notifications];
			});
		} else {
			invoke('get_notifications').then((res) => {
				notifications = res as api.Notification[];
			});
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
