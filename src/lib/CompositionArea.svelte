<script lang="ts">
	export let onPost: (data: { content: string; cw: string | undefined }) => void;
	const data = {
		content: '',
		cw: undefined
	};

	const handlePost = () => {
		onPost(data);
	};
</script>

<div class="w-full">
	<textarea
		bind:value={data.cw}
		class="bg-surface0 w-full min-h-10 text-text p-1"
		placeholder="Content warning (optional)"
	></textarea>

	<textarea
		bind:value={data.content}
		required
		class="bg-surface0 w-full min-h-36 text-text p-1"
		placeholder="Something creative..."
	></textarea>
</div>

<div class="flex flex-row gap-4 items-center">
	<span>Attach</span>
	<span>Emoji</span>
	<span>Poll</span>
	<span>CW</span>

	<div class="grow" />

	<button class="bg-surface0 p-2 rounded-lg min-w-36" on:click={handlePost}>Post</button>
</div>
