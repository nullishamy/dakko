<script lang="ts">
  import * as api from '$lib/api'
	import RenderedContent from '$lib/generic/RenderedContent.svelte';
	import { fullyQualifiedAccount } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { formatDistanceStrict } from 'date-fns';

  export let status: api.Status
  export let onUserSelect: (user: api.Account) => void
  export let onStatusExpand: (status: api.Status) => void

  const { reblog } = status
  const reblogOrStatus = reblog ?? status
	const createdAt = new Date(reblogOrStatus.created_at);

  const pronouns = status.account.fields.find(f => f.name.toLowerCase() === "pronouns")
</script>
	<div class="flex flex-row items-center justify-between">
		<span class="flex flex-row gap-2 items-center flex-wrap min-w-0">
			{#if reblog}
				<img
					class="ml-4 rounded-md"
					height="20"
					width="20"
					src={status.account.avatar}
          alt={`avatar for ${status.account.username}`}
				/>
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]">
					<strong>
						<RenderedContent
							htmlContent={status.account.display_name}
							emojis={status.account.emojis}
						/>
					</strong>
				</span>
				<button
					on:click={() => onUserSelect(status.account)}
					class="text-sm text-nowrap text-blue"
				>
					{fullyQualifiedAccount(status.account)}
				</button>
				<span class="subtext0">boosted</span>
			{:else}
				<img
					height="30"
					width="30"
					class="rounded-md"
					src={status.account.avatar}
          alt={`avatar for ${status.account.username}`}
				/>
				<span class="text-ellipsis whitespace-nowrap overflow-hidden max-w-[85%]">
					<strong>
						<RenderedContent
							htmlContent={status.account.display_name}
							emojis={status.account.emojis}
						/>
					</strong>
				</span>
				<button
					on:click={() => onUserSelect(status.account)}
					class="text-sm text-nowrap text-blue"
				>
					{fullyQualifiedAccount(status.account)}
				</button>
        {#if pronouns}
          <span class="text-sm">({pronouns.value})</span>
        {/if}
			{/if}
		</span>

		<div class="flex flex-row gap-3 items-center whitespace-nowrap">
			<time datetime={createdAt.toISOString()}>
				{formatDistanceStrict(createdAt, new Date(), { addSuffix: true })}
			</time>

			{#if status.visibility === api.StatusVisibility.DIRECT}
				<span title="Direct">
					<Icon
						icon="mdi:envelope"
						width="20"
						height="20"
					/>
				</span>
			{:else if status.visibility === api.StatusVisibility.PRIVATE}
				<span title="Followers-only">
					<Icon
						icon="mdi:lock"
						width="20"
						height="20"
					/>
				</span>
			{:else if status.visibility === api.StatusVisibility.UNLISTED}
				<span title="Unlisted">
					<Icon
						icon="mdi:unlocked"
						width="20"
						height="20"
					/>
				</span>
			{:else if status.visibility === api.StatusVisibility.PUBLIC}
				<span title="Public">
					<Icon
						icon="mdi:public"
						width="20"
						height="20"
					/>
				</span>
			{/if}

			<button on:click={() => onStatusExpand(status)}>
				<Icon
					icon="material-symbols:keyboard-double-arrow-down"
					width="20"
				/>
			</button>
		</div>
	</div>