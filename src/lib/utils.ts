import * as api from '$lib/api'

export function capitalise(str: string): string {
	if (!str) {
		return '';
	}

	const firstChar = str[0].toUpperCase();
	if (str.length == 1) {
		return firstChar;
	}

	const rest = str.substring(1);
	return `${firstChar}${rest}`;
}

export function elipsise(text: string, maxLen: number): string {
	if (text.length + '...'.length >= maxLen) {
		return `${text}...`;
	}

	return text;
}

export function fullyQualifiedAccount(account: api.Account): string {
	const domain = new URL(account.url);
	return `@${account.username}@${domain.host}`;
}
