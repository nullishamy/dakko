export enum InstanceTimeline {
	HOME = 'home',
	PUBLIC = 'public',
	BUBBLE = 'bubble',
	KNOWN = 'known'
}

export interface Instance {
	uri: string;
	title: string;
	description: string;
	email: string;
	version: string;
	thumbnail: string | undefined;
	urls: object | undefined;
	stats: {
		user_count: number;
		status_count: number;
		domain_count: number;
	};
	languages: string[];
	registrations: boolean;
	approval_required: boolean;
	invites_enabled: boolean | undefined;
	configuration: InstanceConfig;
	contact_account: object | undefined;
	rules: object[] | undefined;
}

export interface InstanceConfig {
	statuses: StatusConfig;
	polls: PollConfig | undefined;
}

export interface StatusConfig {
	max_characters: number;
	max_media_attachments: number;
	characters_reserved_per_url: number;
}

export interface PollConfig {
	max_options: number;
	max_characters_per_option: number;
	min_expiration: number;
	max_expiration: number;
}

export interface MarkerData {
	last_read_id: string;
	version: number;
	updated_at: string;
	unread_count: number | undefined;
}

export interface Marker {
	home: MarkerData | undefined;
	notifications: MarkerData | undefined;
}
