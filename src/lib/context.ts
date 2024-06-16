import type { Writable } from 'svelte/store';
import type { Account, Instance, Status, StatusContext, ValidTimeline } from './types';

export interface TimelineContent {
	type: 'timeline';
	timeline: ValidTimeline;
	cachedStatuses: Status[]
	scrollToPostId?: string
}

export interface UserContent {
	type: 'user';
	account: Account;
}

export interface StatusContent {
	type: 'status';

	status: Status;
	statusContext: StatusContext;
	openedId: string;
	onReturn: () => void;
}

export interface NoContent {
	type: 'none';
}

export interface ClientContent {
	type: 'client';
	menu: 'settings' | 'follow_requests' | 'bookmarks';
}

export type MainContent = TimelineContent | UserContent | NoContent | StatusContent | ClientContent;

export interface MainContext {
	instance: Writable<Instance>;
	account: Writable<Account>;
	content: Writable<MainContent>;
}

export const mainContext = {};

export type Theme = "latte" | "frappe" | "macchiato" | "mocha";

export type Accent =
  | "rosewater"
  | "flamingo"
  | "pink"
  | "mauve"
  | "red"
  | "maroon"
  | "peach"
  | "yellow"
  | "green"
  | "teal"
  | "sky"
  | "sapphire"
  | "blue"
  | "lavender";

export interface SettingsContext {
	theme: Writable<Theme>;
	accent: Writable<Accent>;
}

export const settingsContext = {};
