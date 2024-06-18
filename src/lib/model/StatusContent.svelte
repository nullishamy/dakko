<script lang="ts">
	import * as api from '$lib/api';
	import RenderedContent from '$lib/generic/RenderedContent.svelte';
	import { fullyQualifiedAccount } from '../utils';
	import StatusAttachments from './StatusAttachments.svelte';

	export let onUserSelect: (account: api.Account) => void;

	export let status: api.Status;
	const { reblog } = status;

	let showSensitive = false;
	const spoilerText = (reblog ?? status).spoiler_text;
	const handleShowSensitive = () => {
		showSensitive = !showSensitive;
	};

  const pronouns = reblog?.account.fields.find(f => f.name.toLowerCase() === "pronouns")
</script>

{#if reblog}
	<span class="flex flex-row gap-2 items-center">
		<img
			class="rounded-md"
			height="30"
			width="30"
			src={reblog.account.avatar}
      alt={`avatar for ${reblog.account.username}`}
		/>
		<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]">
			<strong>
				<RenderedContent
					htmlContent={reblog.account.display_name}
					emojis={reblog.account.emojis}
				/>
			</strong>
		</span>
		<button
			on:click={() => onUserSelect(reblog.account)}
			class="text-sm text-blue"
		>
			{fullyQualifiedAccount(reblog.account)}
		</button>
        {#if pronouns}
          <span class="text-sm">({pronouns.value})</span>
        {/if}
	</span>

	{#if reblog.spoiler_text && showSensitive}
		<div class="flex flex-col ml-10">
			<em>
				<RenderedContent
					htmlContent={reblog.spoiler_text}
					emojis={reblog.emojis}
				/>
			</em>
			<hr class="bg-accent h-0.5 rounded-lg" />
			<button
				class="text-blue"
				on:click={handleShowSensitive}
			>
				Hide content
			</button>
		</div>
		<div class="text-left ml-10">
			<RenderedContent
				htmlContent={reblog.content}
				emojis={reblog.emojis}
			/>
		</div>
	{:else if reblog.spoiler_text && !showSensitive}
		<div class="flex flex-col ml-10">
			<em>
				<RenderedContent
					htmlContent={reblog.spoiler_text}
					emojis={reblog.emojis}
				/>
			</em>
			<hr class="bg-accent h-0.5 rounded-lg" />
			<button
				class="text-blue"
				on:click={handleShowSensitive}
			>
				Show content
			</button>
		</div>
	{:else}
		<div class="text-left ml-10">
			<RenderedContent
				htmlContent={reblog.content}
				emojis={reblog.emojis}
			/>
		</div>
	{/if}
{:else if status.spoiler_text && showSensitive}
	<div class="flex flex-col ml-10">
		<em>
			<RenderedContent
				htmlContent={status.spoiler_text}
				emojis={status.emojis}
			/>
		</em>
		<hr class="bg-accent h-0.5 rounded-lg" />
		<button
			class="text-blue"
			on:click={handleShowSensitive}
		>
			Hide content
		</button>
	</div>
	<div class="text-left ml-10">
		<RenderedContent
			htmlContent={status.content}
			emojis={status.emojis}
		/>
	</div>
{:else if status.spoiler_text && !showSensitive}
	<div class="flex flex-col ml-10">
		<em>
			<RenderedContent
				htmlContent={status.spoiler_text}
				emojis={status.emojis}
			/>
		</em>
		<hr class="bg-accent h-0.5 rounded-lg" />
		<button
			class="text-blue"
			on:click={handleShowSensitive}
		>
			Show content
		</button>
	</div>
{:else}
	<div class="text-left ml-10">
		<RenderedContent
			htmlContent={status.content}
			emojis={status.emojis}
		/>
	</div>
{/if}

{#if (spoilerText && showSensitive) || !spoilerText}
	<StatusAttachments {status} />
{/if}
