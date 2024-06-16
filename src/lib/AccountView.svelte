<script lang="ts">
	import { onMount } from 'svelte';
	import RenderedContent from './RenderedContent.svelte';
	import type { Account, Status } from './types';
	import { invoke } from '@tauri-apps/api';
	import StatusComponent from './Status.svelte';

	export let account: Account;
	export let isCondensed = false;

	export let onClose = () => {};
	export let onOpen = (account: Account) => {};

	let accountStatuses: Status[] = [];
	onMount(() => {
		invoke('get_statuses', {
			accountId: account.id
		}).then((res) => {
			accountStatuses = res as Status[];
		});
	});
</script>

<div class="w-full p-8 relative">
	<button class="absolute top-2 right-2 text-2xl font-bold" on:click={onClose}> X </button>
	<div class="flex flex-row gap-4 mb-4">
		<div class="w-[75px] h-[75px]">
			<button on:click={() => onOpen(account)}>
				<img
					src={account.avatar}
					alt="Avatar for {account.display_name}"
					width="75"
					height="75"
					class="bg-mantle rounded-md"
				/>
			</button>
		</div>

		<div class="flex flex-col gap-2">
			<span class="text-lg"
				><RenderedContent htmlContent={account.display_name} emojis={account.emojis} /></span
			>
			<span class="text-lg text-blue">@{account.username}</span>
		</div>
	</div>

	<span class="my-4 text-center"
		><RenderedContent htmlContent={account.note} emojis={account.emojis} /></span
	>

	<div class="mt-8 grid grid-cols-3 items-center justify-items-center">
		<span class="font-bold">Posts</span>
		<span class="font-bold">Following</span>
		<span class="font-bold">Followers</span>

		<span>{account.statuses_count}</span>
		<span>{account.following_count}</span>
		<span>{account.followers_count}</span>
	</div>

	{#if !isCondensed}
		<table class="w-full my-8">
			<thead class="">
				<tr>
					<th scope="col" class="px-4"></th>
					<th scope="col" colspan="3" class="px-4"></th>
				</tr>
			</thead>

			<tbody class="m-2">
				{#each account.fields as field}
					<tr>
						{#if field.verified}
							<td class="px-4 py-1 border border-pink w-1/3"
								>{field.name} <span>(verified)</span></td
							>
						{:else}
							<td class="px-4 py-1 border border-pink w-1/3">{field.name}</td>
						{/if}
						<td
							class="px-4 py-1 border border-pink overflow-ellipsis whitespace-nowrap overflow-hidden"
							>{@html field.value}</td
						>
					</tr>
				{/each}
			</tbody>
		</table>

		{#if !accountStatuses.length}
			<span>Loading posts..</span>
		{/if}

		<div class="flex flex-col gap-4">
			{#each accountStatuses as status}
				<StatusComponent
					{status}
					onOpen={() => {
						throw new TypeError('not implemented');
					}}
				/>
			{/each}
		</div>
	{/if}
</div>
