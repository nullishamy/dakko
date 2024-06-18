<script lang="ts">
	import NotificationPanel from '$lib/pane/NotificationPanel.svelte';
	import Sidebar from '$lib/pane/Sidebar.svelte';
	import * as api from '$lib/api';
	import {
		mainContext,
		type MainContext,
		type MainContent,
		type SettingsContext,
		settingsContext,
		type Theme,
		type Accent,
	} from '$lib/context';
	import { onMount, setContext } from 'svelte';
	import { writable } from 'svelte/store';
	import MainContentComponent from '$lib/pane/MainContent.svelte';
	import { Filter, Filters } from '../lib/filtering';

	const instance = writable<api.Instance>();
	const account = writable<api.Account>();
	const content = writable<MainContent>();

	setContext<MainContext>(mainContext, {
		instance,
		account,
		content
	});

	let theme = localStorage.getItem('theme') ?? 'latte';
	let accent = localStorage.getItem('accent') ?? 'pink';

	const existingFilters = JSON.parse(localStorage.getItem('filters') ?? '[]')

	const filters = new Filters();
	console.log(existingFilters);
	
	filters.filters = existingFilters.map((d: any) => {
		return new Filter(d.name, d.applicationPredicate, d.code)
	})

	const filtersWritable = writable(filters)
	setContext<SettingsContext>(settingsContext, {
		theme: writable(theme as Theme),
		accent: writable(accent as Accent),
		filters: filtersWritable
	});

	filtersWritable.subscribe(newFilters => {
		localStorage.setItem("filters", JSON.stringify(newFilters.filters))
	})

	document.body.classList.add(theme);
	document.body.style.setProperty('--dakko-accent', `var(--ctp-${accent})`);

	let loginState: api.LoginStatus | undefined = undefined;

	let authURL: string | undefined;
	let instanceURL = 'https://labyrinth.zone';

	const handleSubmit = async () => {
		await api.setInstance(instanceURL);
		authURL = await api.fetchLoginURL();
		window.location.replace(authURL as string);
	};

	onMount(async () => {
		loginState = await api.fetchLoginState();

		if (loginState == api.LoginStatus.LOGGED_IN) {
			content.set({
				type: 'timeline',
				timeline: api.InstanceTimeline.HOME,
				cachedStatuses: []
			});
			// content.set({
			// 	type: 'client',
			// 	menu: 'settings'
			// });

			console.time('instance_fetch');
			const fetchedInstance = await api.fetchInstance();
			instance.set(fetchedInstance);
			console.timeEnd('instance_fetch');

			console.time('user_fetch');
			const fetchedUser = await api.fetchSelf();
			account.set(fetchedUser);
			console.timeEnd('user_fetch');
		}
	});

	function navigateToLoginPage() {
		window.location.replace(authURL as string);
	}
</script>

{#if loginState == api.LoginStatus.LOGGED_IN}
	<div class="grid grid-cols-7 py-2 h-full overflow-hidden">
		<section class="border-r-accent border-r-[1px] p-1">
			<Sidebar />
		</section>

		<main class="col-span-4 border-x-accent border-x-[1px] px-2 overflow-y-scroll hide-scrollbar">
			<MainContentComponent />
		</main>

		<section
			class="col-span-2 border-l-accent border-l-[1px] px-2 overflow-y-scroll hide-scrollbar"
		>
			<NotificationPanel />
		</section>
	</div>
{:else if loginState == api.LoginStatus.LOGIN_EXPIRED && authURL}
	<button on:click={navigateToLoginPage}>Login</button>
{:else if loginState === undefined}
	<span>Checking login state...</span>
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
