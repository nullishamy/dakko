<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import NotificationComponent from "./Notification.svelte";
  import { NotificationType, type Notification } from "./types";

  let notifications: Notification[] = []

  invoke('get_notifications').then(res => {
    notifications = res as Notification[]
  })
</script>

<h2 class="bg-mantle p-1 text-xl">Notifications</h2>
<div class="mt-2 flex flex-col gap-2">
  {#each notifications as notification}
    <NotificationComponent {notification}/>
  {/each}
</div>