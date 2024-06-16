import type { Account } from "./account";

export interface Emoji {
	shortcode: string;
	static_url: string;
	url: string;
	visible_in_picker: boolean;
	category: string | undefined;
}

export interface Status {
	id: string;
	uri: string;
	url: string | undefined;
	account: Account;
	in_reply_to_id: string | undefined;
	in_reply_to_account_id: string | undefined;
	reblog: Status | undefined;
	content: string;
	plain_content: string | undefined;
	created_at: string;
	edited_at: string;
	emojis: Emoji[];
	replies_count: number;
	reblogs_count: number;
	favourites_count: number;
	reblogged: boolean | undefined;
	favourited: boolean | undefined;
	muted: boolean | undefined;
	sensitive: boolean;
	spoiler_text: string;
	visibility: StatusVisibility;
	media_attachments: Attachment[];
	mentions: unknown[];
	tags: unknown[];
	card: unknown;
	poll: unknown;
	application: unknown;
	language: string | undefined;
	pinned: boolean | undefined;
	emoji_reactions: unknown[] | undefined;
	quote: boolean;
	bookmarked: boolean | undefined;
}

export enum StatusVisibility {
	PUBLIC = 'public',
	UNLISTED = 'unlisted',
	PRIVATE = 'private',
	DIRECT = 'direct'
}

export enum AttachmentType {
	IMAGE = 'image',
	GIFV = 'gifv',
	VIDEO = 'video',
	AUDIO = 'audio',
	UNKNOWN = 'unknown'
}

export interface Attachment {
	id: string;
	type: AttachmentType;
	url: string;
	remote_url: string | undefined;
	preview_url: string | undefined;
	text_url: string | undefined;
	meta: unknown;
	description: string | undefined;
	blurhash: string | undefined;
}

export interface StatusContext {
	ancestors: Status[];
	descendants: Status[];
}