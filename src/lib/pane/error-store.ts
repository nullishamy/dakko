import { writable, type Writable } from "svelte/store";

export const errors: Writable<string[]> = writable([])