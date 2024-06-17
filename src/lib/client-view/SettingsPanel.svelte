<script lang="ts">
	import { getContext } from 'svelte';
	import {
		type SettingsContext,
		settingsContext,
		type Theme,
		type Accent,
		Filter
	} from '$lib/context';
	import Icon from '@iconify/svelte';

	const { theme, accent, filters } = getContext<SettingsContext>(settingsContext);
	const handleThemeChange = (value: string) => {
		if (value === 'current') {
			return;
		}

		document.body.classList.remove(value, $theme);
		document.body.classList.add(value);
		theme.set(value as Theme);
		localStorage.setItem('theme', value);
	};

	const handleAccentChange = (value: string) => {
		if (value === 'current') {
			return;
		}

		accent.set(value as Accent);
		localStorage.setItem('accent', value);
		document.body.style.setProperty('--dakko-accent', `var(--ctp-${value})`);
	};

	const handleFilterChange = (
		filter: Filter,
		e: KeyboardEvent & {
			currentTarget: EventTarget & HTMLElement;
		},
		field: 'predicate' | 'code' | 'name'
	) => {
		const newValue = e.currentTarget.textContent;
		if (!newValue) {
			return;
		}

		if (field === 'predicate') {
			filter.applicationPredicate = newValue;
		} else if (field === 'code') {
			filter.code = newValue;
		} else {
			filter.name = newValue;
		}

		filters.set($filters);
	};

	const makeNewFilter = () => {
		filters.set($filters.addFilter(new Filter('name', 'true', 'false')));
	};

	const removeFilter = (filter: Filter) => {
		filters.set($filters.removeFilter(filter));
	};
</script>

<section class="flex flex-col items-center m-4 mx-12 gap-8">
	<h1 class="text-2xl">Settings</h1>

	<div class="w-full border border-accent rounded-md p-4">
		<label
			for="theme"
			class="text-lg underline"
		>
			Theme
		</label>

		<select
			name="theme"
			class="bg-mantle rounded-md py-1 appearance-none"
			on:change={(e) => handleThemeChange(e.currentTarget.value)}
		>
			<option
				class="bg-mantle"
				value="current"
			>
				Select theme (current: {$theme}) <Icon icon="mdi:keyboard-arrow-down" />
			</option>
			<option value="latte">Latte (light)</option>
			<option value="mocha">Mocha (dark)</option>
			<option value="macchiato">Macchiato</option>
			<option value="frappe">Frappe</option>
		</select>
	</div>

	<div class="w-full border border-accent rounded-md p-4">
		<label
			for="accent"
			class="text-lg underline"
		>
			Accent
		</label>
		<select
			name="accent"
			class="bg-mantle rounded-md py-1 appearance-none"
			on:change={(e) => handleAccentChange(e.currentTarget.value)}
		>
			<option
				class="bg-mantle"
				value="current"
			>
				Select accent (current: {$accent}) <Icon icon="mdi:keyboard-arrow-down" />
			</option>

			<option value="rosewater">Rosewater</option>
			<option value="flamingo">Flamingo</option>
			<option value="pink">Pink</option>
			<option value="mauve">Mauve</option>
			<option value="red">Red</option>
			<option value="maroon">Maroon</option>
			<option value="peach">Peach</option>
			<option value="yellow">Yellow</option>
			<option value="green">Green</option>
			<option value="teal">Teal</option>
			<option value="sky">Sky</option>
			<option value="sapphire">Sapphire</option>
			<option value="blue">Blue</option>
			<option value="lavender">Lavender</option>
		</select>
	</div>

	<div class="w-full border border-accent rounded-md p-4 flex flex-col gap-4">
		<span class="text-lg underline">Filters</span>
		{#if !$filters.filters.length}
			<span>No filters setup, click New to setup a new filter</span>
		{/if}

		{#each $filters.filters as filter}
			<div class="flex flex-col gap-4 border border-accent p-4 rounded-md relative">
				<button
					on:click={() => removeFilter(filter)}
					class="absolute top-4 right-3 bg-red px-3 py-1 rounded-md text-base"
				>
					Delete
				</button>
				<span
					role="textbox"
					tabindex="0"
					contenteditable="plaintext-only"
					class="underline mb-2 w-4/5"
					on:keyup={(e) => handleFilterChange(filter, e, 'name')}
				>
					{filter.name}
				</span>
				<div class="flex flex-row items-center gap-2">
					<span>Apply when</span>
					<code
						on:keyup={(e) => handleFilterChange(filter, e, 'predicate')}
						contenteditable="plaintext-only"
						class="bg-mantle p-2 rounded-md text-sm"
					>
						{filter.applicationPredicate}
					</code>
				</div>
				<div class="flex flex-row items-center gap-2">
					<span>Filter out</span>
					<code
						on:keyup={(e) => handleFilterChange(filter, e, 'code')}
						contenteditable="plaintext-only"
						class="bg-mantle p-2 rounded-md text-sm"
					>
						{filter.code}
					</code>
				</div>
			</div>
		{/each}

		<div class="flex flex-row items-center gap-4">
			<button
				on:click={makeNewFilter}
				class="bg-mantle px-3 py-1 rounded-md border border-accent"
			>
				New
			</button>
		</div>
	</div>
</section>
