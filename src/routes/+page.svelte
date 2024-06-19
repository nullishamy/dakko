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
		type Accent
	} from '$lib/context';
	import { onMount, setContext } from 'svelte';
	import { writable } from 'svelte/store';
	import MainContentComponent from '$lib/pane/MainContent.svelte';
	import { Filter, Filters } from '../lib/filtering';
	import { logger } from '$lib/log';
	import Bootstrap from '$lib/pane/Bootstrap.svelte';
	import { type BootstrapData } from '$lib/pane/bootstrap';
	import { WebviewWindow } from '@tauri-apps/api/window';

	logger.info('frontend starting up');

	const instance = writable<api.Instance>();
	const account = writable<api.Account>();
	const content = writable<MainContent>();

	setContext<MainContext>(mainContext, {
		instance,
		account,
		content
	});

	let themeValue = localStorage.getItem('theme') ?? 'latte';
	let accentValue = localStorage.getItem('accent') ?? 'pink';

	// Set the theme as early as possible, to avoid FOUC
	document.body.classList.add(themeValue);
	document.body.style.setProperty('--dakko-accent', `var(--ctp-${accentValue})`);

	const theme = writable(themeValue as Theme);
	const accent = writable(accentValue as Accent);
	theme.subscribe((newTheme) => {
		logger.info('setting new theme', newTheme);
		localStorage.setItem('theme', newTheme);
	});

	accent.subscribe((newAccent) => {
		logger.info('setting new accent', newAccent);
		localStorage.setItem('accent', newAccent);
	});

	const existingFilters = JSON.parse(localStorage.getItem('filters') ?? '[]');

	const filters = new Filters();
	filters.filters = existingFilters.map((d: any) => {
		return new Filter(d.name, d.applicationPredicate, d.code);
	});

	const filtersWritable = writable(filters);
	filtersWritable.subscribe((newFilters) => {
		logger.info('setting new filters', newFilters);
		localStorage.setItem('filters', JSON.stringify(newFilters.filters));
	});

	logger.debug('existing settings:', { theme: themeValue, accent: accentValue, filters });
	setContext<SettingsContext>(settingsContext, {
		theme,
		accent,
		filters: filtersWritable
	});

	let loginState: api.LoginStatus | undefined = undefined;

	const init = async () => {
		logger.debug('starting initial requests');
		loginState = await api.fetchLoginState();
		logger.debug('got login state:', loginState);

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
	};

	onMount(async () => {
		await init();
	});

	const onBootstrap = (data: BootstrapData) => {
		const webview = new WebviewWindow('auth', {
			url: data.authURL
		});
		webview.once('tauri://created', function () {
			logger.info('authorization webview created, awaiting completion');
		});
		webview.once('tauri://error',  (e) => {
			logger.info('error in the authorization webview', e);
		});

		webview.listen('auth-complete', () => {
			logger.info('authorization complete, closing webview and initializing state');
			webview.close();
			loginState = api.LoginStatus.LOGGED_IN;
			init()
		});
	};
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
{:else if loginState === undefined}
	<span>Checking login state...</span>
{:else}
	<Bootstrap onCompletion={onBootstrap} />
{/if}
