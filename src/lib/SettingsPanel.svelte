<script lang="ts">
	import { getContext } from 'svelte';
	import { type SettingsContext, settingsContext, type Theme, type Accent } from './context';
	import Icon from '@iconify/svelte';

	const { theme, accent } = getContext<SettingsContext>(settingsContext);
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
</script>

<section class="flex flex-col items-center m-4 mx-12 gap-8">
	<h1 class="text-2xl">Settings</h1>

	<div class="w-full border border-accent rounded-md p-4">
		<label for="theme">Theme</label>

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
		<label for="accent">Accent</label>
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
</section>
