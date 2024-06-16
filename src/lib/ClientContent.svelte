<script lang="ts">
	import Bookmarks from './Bookmarks.svelte';
import FollowRequests from './FollowRequests.svelte';
	import SettingsPanel from './SettingsPanel.svelte';
	import type { ClientContent } from './context';

	export let content: ClientContent;
</script>

{#if content.menu === 'settings'}
	<SettingsPanel />
{:else if content.menu === 'follow_requests'}
	<FollowRequests />
{:else if content.menu === 'bookmarks'}
	<Bookmarks />
{/if}
