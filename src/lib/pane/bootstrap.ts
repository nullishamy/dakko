export enum InstanceType {
	MASTODON = "Mastodon",
	PLEROMA = "Pleroma",
	FRIENDICA = "Friendica",
	FIREFISH = "Firefish",
	GOTOSOCIAL = "Gotosocial",
}

export interface BootstrapData {
	instanceURL: string;
	authURL: string;
	instanceType: InstanceType
}
