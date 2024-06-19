type LogFn = (...data: unknown[]) => void;

export interface Logger {
	debug: LogFn;
	info: LogFn;
	warn: LogFn;
	error: LogFn;
}

export const logger: Logger = {
	debug: console.debug,
	info: console.info,
	warn: console.warn,
	error: console.error
};
