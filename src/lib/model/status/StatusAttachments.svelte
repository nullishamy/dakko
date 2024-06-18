<script lang="ts">
	import * as api from '$lib/api';

	export let status: api.Status;
	const { reblog } = status;

	const reblogOrStatus = reblog ?? status;
	const imagesAreSensitive = reblogOrStatus.sensitive;

	let showSensitiveImages = false;
</script>

{#if status.media_attachments.length}
	<div
		class="flex flex-row overflow-x-scroll gap-4 p-1 items-center justify-items-center justify-center border border-accent mt-2 rounded-md h-[32rem]"
	>
		{#each status.media_attachments as attachment}
			{#if attachment.type === api.AttachmentType.IMAGE}
				{#if (showSensitiveImages && imagesAreSensitive) || !imagesAreSensitive}
					<img
						class="h-auto w-auto max-h-full"
						src={attachment.remote_url}
						alt={attachment.description}
						title={attachment.description}
					/>
				{:else}
					<button
						class="w-full h-full text-3xl text-red underline"
						on:click={() => {
							showSensitiveImages = true;
						}}
					>
						Image marked as sensitive, click to show
					</button>
				{/if}
			{:else}
				<span>Unsupported media type {attachment.type}</span>
				<a href={attachment.remote_url}>Click to view externally</a>
			{/if}
		{/each}
	</div>
{/if}
