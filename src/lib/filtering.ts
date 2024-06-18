import * as api from '$lib/api'

export type FilterTarget = 'status';

export interface FilterContext {
	status: api.Status;
	target: FilterTarget;
}

const utils = {
	anyOf: (...preds: boolean[]) => {
		for (const pred of preds) {
			if (pred) {
				return true;
			}
		}

		return false;
	},
	keyword: (ctx: FilterContext, word: string) => {
		return ctx.status.content.includes(word);
	},
	cwKeyword: (ctx: FilterContext, word: string) => {
		return ctx.status.spoiler_text.includes(word);
	},
	usernameContains: (ctx: FilterContext, needle: string) => {
		return ctx.status.account.username.includes(needle)
	},
	warnIf: (cond: boolean): FilterExecutionResult => {
		return cond ? 'warning' : 'show';
	},
	disabled: (): boolean => {
		return false;
	}
};

function withContext<A extends unknown[], R>(
	ctx: FilterContext,
	f: (ctx: FilterContext, ...args: A) => R
): (...args: A) => R {
	return (...args: A) => f(ctx, ...args);
}

export class Filter {
	name: string;
	applicationPredicate: string;
	code: string;

	private executeCode(code: string, values: Record<string, unknown>): unknown {
		const func = Function(...Object.keys(values), `"use strict";return (${code});`);
		return func(...Object.values(values));
	}

	constructor(name: string, applicationPredicate: string, code: string) {
		this.name = name;
		this.applicationPredicate = applicationPredicate;
		this.code = code;
	}

	applicable(ctx: FilterContext): boolean {
		try {
			const execResult = this.executeCode(this.applicationPredicate, {
				ctx,
				disabled: utils.disabled
			});

			return !!execResult;
		} catch (err) {
			console.error(
				`Error running filter ${JSON.stringify(this, undefined, 2)} defaulting to applicable:`,
				err
			);
			return true;
		}
	}

	execute(ctx: FilterContext): FilterExecutionResult {
		try {
			const execResult = this.executeCode(this.code, {
				ctx,
				anyOf: utils.anyOf,
				warnIf: utils.warnIf,
				cwKeyword: withContext(ctx, utils.cwKeyword),
				keyword: withContext(ctx, utils.keyword),
				usernameContains: withContext(ctx, utils.usernameContains)
			});

			// We will allow booleans, which simply show or hide a post
			if (typeof execResult === 'boolean') {
				return execResult ? 'hidden' : 'show';
			}

			if (
				typeof execResult !== 'string' ||
				!(execResult === 'warning' || execResult === 'hidden' || execResult === 'show')
			) {
				console.error('Filter returned invalid result:', JSON.stringify(execResult, undefined, 2));
				return 'show';
			}

			return execResult;
		} catch (err) {
			console.error(
				`Error running filter ${JSON.stringify(this, undefined, 2)} defaulting to show:`,
				err
			);
			return 'show';
		}
	}
}

export type FilterExecutionResult = 'warning' | 'hidden' | 'show';

export class Filters {
	filters: Filter[];
	constructor() {
		this.filters = [];
	}

	addFilter(filter: Filter): Filters {
		this.filters.push(filter);
		return this;
	}

	removeFilter(filter: Filter): Filters {
		this.filters = this.filters.filter((f) => f !== filter);
		return this;
	}

	filterStatus(status: api.Status): [FilterExecutionResult, Filter | undefined] {
		const ctx: FilterContext = {
			status,
			target: 'status'
		};

		for (const filter of this.filters) {
			if (filter.applicable(ctx)) {
				return [filter.execute(ctx), filter];
			}
		}

		return ['show', undefined];
	}
}