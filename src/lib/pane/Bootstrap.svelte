<script lang="ts">
	import * as api from '$lib/api';
	import type { BootstrapData } from './bootstrap';

	export let onCompletion: (data: BootstrapData) => void;

	let authURL: string | undefined;
	let instanceURL = 'https://labyrinth.zone';

	const handleSubmit = async () => {
		await api.setInstance(instanceURL);
		authURL = await api.fetchLoginURL();
		onCompletion({
			instanceURL,
			authURL
		});
	};
</script>

<form on:submit={handleSubmit} class="max-w-screen-lg m-auto h-full flex flex-col items-center">
  <h1 class="text-2xl">Welcome to Dakko!</h1>
  <span class="text-xl mb-8">Lets get you setup</span>

	<label for="url" class="text-lg">Your instance URL:</label>
	<input
    bind:value={instanceURL}
    class="w-2/3 rounded-md"
		type="url"
		name="url"
	/>

	<button type="submit" class="mt-8 text-lg rounded-md py-1 px-5 border border-text bg-mantle">Continue</button>
</form>
