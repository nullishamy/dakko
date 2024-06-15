<script lang="ts">
	import NotificationPanel from '$lib/NotificationPanel.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import Timeline from '$lib/Timeline.svelte';
	import { invoke } from '@tauri-apps/api';
	import { ValidTimeline, type Account, type Instance } from '$lib/types';
	import { mainContext, type Context, type MainContent} from '$lib/context';
	import { setContext } from 'svelte';
	import { writable } from 'svelte/store';
	import MainContentComponent from '$lib/MainContent.svelte';

	const instance = writable<Instance>();
	const account = writable<Account>();
	const content = writable<MainContent>();

	setContext<Context>(mainContext, {
		instance,
		account,
		content
	});

	let isLoggedIn: boolean | undefined = undefined;
	let authURL: string | undefined = undefined;

	const goToHome = () => {
		content.set({
			type: 'timeline',
			timeline: ValidTimeline.HOME
		})
	}

	invoke('is_logged_in')
		.then((res) => {
			isLoggedIn = res as boolean;
			if (isLoggedIn) {
				console.time('instance_fetch');
				invoke('get_instance')
					.then((res) => {
						instance.set(res as Instance);
						console.timeEnd('instance_fetch');
					})
					.catch((e) => console.error(e));

				console.time('user_fetch');
				invoke('get_user')
					.then((res) => {
						account.set(res as Account);
						content.set({
							type: 'user',
							account: res as Account
						})
						console.timeEnd('user_fetch');
					})
					.catch((e) => console.error(e));
			} else {
				invoke('login').then((res) => {
					authURL = res as string;
				});
			}
		})
		.catch((e) => console.error(e));

	function navigateToLoginPage() {
		window.location.replace(authURL as string);
	}
</script>

{#if isLoggedIn}
	<div class="grid grid-cols-7 py-2 h-full overflow-hidden">
		<nav class="col-span-7 px-2">
			<button on:click={goToHome}>Home</button>
		</nav>

		<section class="border-r-pink border-r-[1px] p-1 overflow-scroll">
			<Sidebar />
		</section>

		<main class="col-span-4 border-x-pink border-x-[1px] px-2 overflow-scroll">
			<MainContentComponent />
		</main>

		<section class="col-span-2 border-l-pink border-l-[1px] px-2 overflow-scroll">
			<NotificationPanel />
		</section>
	</div>
{:else if authURL}
	<button on:click={navigateToLoginPage}>Login</button>
{:else}
	<span>Please wait</span>
{/if}
