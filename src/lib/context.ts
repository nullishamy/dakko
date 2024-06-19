import type { Writable } from 'svelte/store';
import * as api from '$lib/api';
import type { Filters } from './filtering';

export interface TimelineContent {
	type: 'timeline';
	timeline: api.InstanceTimeline;
	cachedStatuses: api.Status[];
	scrollToPostId?: string;
}

export interface UserContent {
	type: 'user';
	account: api.Account;
}

export interface StatusContent {
	type: 'status';

	status: api.Status;
	statusContext: api.StatusContext;
	openedId: string;
	onReturn: () => void;
}

export interface NoContent {
	type: 'none';
}

export interface ErrorContent {
	type: 'error';
	message: string
}

export interface ClientContent {
	type: 'client';
	menu: 'settings' | 'follow_requests' | 'bookmarks';
}

export type MainContent = TimelineContent | UserContent | NoContent | StatusContent | ClientContent | ErrorContent;

export interface MainContext {
	instance: Writable<api.Instance>;
	account: Writable<api.Account>;
	content: Writable<MainContent>;
}

export const mainContext = {};

export type Theme = 'latte' | 'frappe' | 'macchiato' | 'mocha';

export type Accent =
	| 'rosewater'
	| 'flamingo'
	| 'pink'
	| 'mauve'
	| 'red'
	| 'maroon'
	| 'peach'
	| 'yellow'
	| 'green'
	| 'teal'
	| 'sky'
	| 'sapphire'
	| 'blue'
	| 'lavender';

export interface SettingsContext {
	theme: Writable<Theme>;
	accent: Writable<Accent>;
	filters: Writable<Filters>;
	font: Writable<string>;
}

export const settingsContext = {};
