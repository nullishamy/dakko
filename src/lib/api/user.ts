import type { Account } from './account';
import type { Status } from './content';

export interface Relationship {
	id: string;
	following: boolean;
	followed_by: boolean;
	blocking: boolean;
	blocked_by: boolean;
	muting: boolean;
	muting_notifications: boolean;
	requested: boolean;
	domain_blocking: boolean;
	showing_reblogs: boolean;
	endorsed: boolean;
	notifying: boolean;
	// Friendica returns null as note.
	note: string | undefined;
}

export enum NotificationType {
	FOLLOW = 'follow',
	FOLLOW_REQUEST = 'follow_request',
	MENTION = 'mention',
	REBLOG = 'reblog',
	FAVOURITE = 'favourite',
	POLl_VOTE = 'poll_vote',
	POLL_EXPIRED = 'poll_expired',
	STATUS = 'status',
	REACTION = 'reaction',
	UPDATE = 'update',
	MOVE = 'move',
	ADMIN_SIGNUP = 'admin_signup',
	ADMIN_REPORT = 'admin_report',
	GROUP_INVITED = 'group_invited',
	APP = 'app',
	UNKNOWN = 'unknown'
}

export enum LoginStatus {
	LOGGED_IN = 'LoggedIn',
	INSTANCE_UNKNOWN = 'InstanceUnknown',
	LOGIN_EXPIRED = 'LoginExpired'
}

export interface Notification {
	account: Account | undefined;
	created_at: string;
	id: string;
	status: Status | undefined;
	reaction: unknown;
	target: Account | undefined;
	type: NotificationType;
}
