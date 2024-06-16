import type { Emoji } from './content';

export interface AccountField {
	name: string;
	value: string;
	verified_at: string | undefined;
	verified: boolean | undefined;
}

export interface Account {
	id: string;
	username: string;
	acct: string;
	display_name: string;
	locked: boolean;
	discoverable: boolean | undefined;
	group: boolean | undefined;
	noindex: boolean | undefined;
	moved: Account | undefined;
	suspended: boolean | undefined;
	limited: boolean | undefined;
	created_at: string;
	followers_count: number;
	following_count: number;
	statuses_count: number;
	note: string;
	url: string;
	avatar: string;
	avatar_static: string;
	header: string;
	header_static: string;
	emojis: Emoji[];
	fields: AccountField[];
	bot: boolean;
	source: unknown;
	role: unknown;
	mute_expires_at: string | undefined;
}
