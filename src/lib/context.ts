import type { Writable } from 'svelte/store';
import type { Account, Instance, Status, StatusContext, ValidTimeline } from './types';

export interface TimelineContent {
	type: 'timeline'
	timeline: ValidTimeline
}

export interface UserContent {
	type: 'user'
	account: Account
}

export interface StatusContent {
	type: 'status'

	status: Status
	statusContext: StatusContext
	openedId: string
	onReturn: () => void
}

export interface NoContent {
	type: 'none'
}

export type MainContent = TimelineContent | UserContent | NoContent | StatusContent

export interface Context {
	instance: Writable<Instance>;
	account: Writable<Account>;
	content: Writable<MainContent>;
}

export const mainContext = {};
