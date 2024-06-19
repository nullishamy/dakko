<script lang="ts">
	import * as api from '$lib/api';
	import { InstanceType, type BootstrapData } from './bootstrap';

	export let onCompletion: (data: BootstrapData) => void;

	let authURL: string | undefined;
	let instanceURL = 'https://labyrinth.zone';
	let instanceType: InstanceType = InstanceType.PLEROMA;

	const handleSubmit = async () => {
		await api.setInstance(instanceURL, instanceType);
		authURL = await api.fetchLoginURL();
		onCompletion({
			instanceURL,
			authURL,
			instanceType
		});
	};
</script>

<form
	on:submit={handleSubmit}
	class="max-w-screen-lg m-auto h-full flex flex-col items-center"
>
	<h1 class="text-2xl mb-8">
		Welcome to Dakko!
		<span class="text-xl">Lets get you setup</span>
	</h1>

	<label
		for="url"
		class="text-lg"
	>
		Your instance URL:
	</label>
	<input
		bind:value={instanceURL}
		class="w-2/3 rounded-md mb-4"
		type="url"
		name="url"
	/>

	<label
		for="url"
		class="text-lg"
	>
		Your instance URL:
	</label>
	<select
		name="instanceType"
		bind:value={instanceType}
	>
		<option value="">--Choose an option--</option>
		<option value="Mastodon">Mastodon</option>
		<option value="Pleroma">Pleroma</option>
		<option value="Firefish">Firefish</option>
		<option value="Friendica">Friendica</option>
	</select>

	<button
		type="submit"
		class="mt-8 text-lg rounded-md py-1 px-5 border border-text bg-mantle"
	>
		Continue
	</button>
</form>
