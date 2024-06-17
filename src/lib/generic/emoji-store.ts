import * as api from '$lib/api'
import { writable } from "svelte/store";

export const emojiStore = writable<api.CustomEmoji[]>()