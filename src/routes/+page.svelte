<script lang="ts">
	import NotificationPanel from '$lib/NotificationPanel.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import Timeline from '$lib/Timeline.svelte';
	import { invoke } from '@tauri-apps/api';
	import { LoginStatus, ValidTimeline, type Account, type Instance } from '$lib/types';
	import {
		mainContext,
		type MainContext,
		type MainContent,
		type SettingsContext,
		settingsContext,

		type Theme,

		type Accent


	} from '$lib/context';
	import { onMount, setContext } from 'svelte';
	import { writable } from 'svelte/store';
	import MainContentComponent from '$lib/MainContent.svelte';

	const instance = writable<Instance>();
	const account = writable<Account>();
	const content = writable<MainContent>();

	setContext<MainContext>(mainContext, {
		instance,
		account,
		content
	});

	let theme = localStorage.getItem('theme') ?? 'latte';
	let accent = localStorage.getItem('accent') ?? 'pink';

	setContext<SettingsContext>(settingsContext, {
		theme: writable(theme as Theme),
		accent: writable(accent as Accent)
	});
	document.body.classList.add(theme);
	document.body.style.setProperty('--dakko-accent', `var(--ctp-${accent})`);

	let loginState: LoginStatus | undefined = undefined;

	let authURL: string | undefined;
	let instanceURL = 'https://labyrinth.zone';

	const handleSubmit = () => {
		invoke('set_instance', {
			url: instanceURL
		}).then(() => {
			invoke('login')
				.then((res) => {
					authURL = res as string;
					window.location.replace(authURL as string);
				})
				.catch(console.error);
		});
	};

	onMount(() => {
		invoke('login_state')
			.then((res) => {
				loginState = res as LoginStatus;
				if (loginState == LoginStatus.LOGGED_IN) {
					console.time('instance_fetch');
					invoke('get_instance')
						.then((res) => {
							instance.set(res as Instance);
							content.set({
								type: 'timeline',
								timeline: ValidTimeline.HOME,
								cachedStatuses: []
							});
							// content.set({
							// 	type: 'settings',
							// });
							console.timeEnd('instance_fetch');
						})
						.catch((e) => console.error(e));

					console.time('user_fetch');
					invoke('get_user')
						.then((res) => {
							account.set(res as Account);
							// content.set({
							// 	type: 'user',
							// 	account: res as Account
							// })
							console.timeEnd('user_fetch');
						})
						.catch((e) => console.error(e));
				}
			})
			.catch((e) => console.error(e));
	});

	function navigateToLoginPage() {
		window.location.replace(authURL as string);
	}
</script>

{#if loginState == LoginStatus.LOGGED_IN}
	<div class="grid grid-cols-7 py-2 h-full overflow-hidden">
		<section class="border-r-accent border-r-[1px] p-1">
			<Sidebar />
		</section>

		<main class="col-span-4 border-x-accent border-x-[1px] px-2 overflow-y-scroll hide-scrollbar">
			<MainContentComponent />
		</main>

		<section class="col-span-2 border-l-accent border-l-[1px] px-2 overflow-y-scroll hide-scrollbar">
			<NotificationPanel />
		</section>
	</div>
{:else if loginState == LoginStatus.LOGIN_EXPIRED && authURL}
	<button on:click={navigateToLoginPage}>Login</button>
{:else}
	<form on:submit={handleSubmit}>
		<label for="url">Your instance URL:</label>
		<input
			on:change={(e) => {
				instanceURL = e.currentTarget.value;
			}}
			type="url"
			name="url"
			value={instanceURL}
		/>
		<button type="submit">Confirm</button>
	</form>
{/if}
