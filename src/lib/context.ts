import type { Writable } from 'svelte/store';
import type { Account, Instance } from './types';

export interface Context {
	instance: Writable<Instance>;
	account: Writable<Account>;
}

export const mainContext = {};
