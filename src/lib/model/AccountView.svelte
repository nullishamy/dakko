<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import RenderedContent from '$lib/generic/RenderedContent.svelte';
	import * as api from '$lib/api';
	import StatusComponent from './Status.svelte';
	import Icon from '@iconify/svelte';
	import { type MainContext, mainContext } from '$lib/context';
	import { openStatus } from '../utils';

	export let account: api.Account;
	export let isCondensed = false;

	export let onClose = () => {};
	export let onOpen = (account: api.Account) => {};

	const handleStatusOpen = async (status: api.Status) => {
		await openStatus(status, content, () => {
			content.set({
				type: 'user',
				account
			});
		});
	};

	const { content } = getContext<MainContext>(mainContext);

	let accountStatuses: api.Status[] = [];
	let relationship: api.Relationship = {
		id: '',
		following: false,
		followed_by: false,
		blocking: false,
		blocked_by: false,
		muting: false,
		muting_notifications: false,
		requested: false,
		domain_blocking: false,
		showing_reblogs: false,
		endorsed: false,
		notifying: false,
		note: undefined
	};

	onMount(async () => {
		relationship = (await api.fetchRelationships(account.id))[0];
		console.log(relationship)
		if (!isCondensed) {
			accountStatuses = await api.fetchStatuses(account.id);
		}
	});

	const toggleFollow = async () => {
		if (relationship.following) {
			relationship = await api.unfollowUser(account.id);
		} else {
			relationship = await api.followUser(account.id);
		}
	};
</script>

<div class="w-full p-8 relative">
	<button
		class="absolute top-2 right-2 text-2xl font-bold"
		on:click={onClose}
	>
		X
	</button>
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

		<div class="flex flex-col gap-1 mt-2">
			<span class="text-lg flex flex-row items-center gap-2">
				<RenderedContent
					htmlContent={account.display_name}
					emojis={account.emojis}
				/>
				{#if account.locked}
					<Icon
						icon="material-symbols:lock"
						height="15"
						width="15"
					/>
				{/if}
			</span>
			<span class="text-lg text-blue">@{account.username}</span>
		</div>
	</div>

	<div class="my-8 flex flex-row gap-8">
		<button
			class="py-0.5 px-3 border border-accent rounded-md"
			on:click={toggleFollow}
		>
			{#if relationship.requested}
				Requested!
			{:else if relationship.following}
				Following
			{:else}
				Follow
			{/if}
		</button>
		<button class="py-0.5 px-3 border border-accent rounded-md">
			{#if relationship.muting}
				Muted
			{:else}
				Mute
			{/if}
		</button>
		<button class="py-0.5 px-3 border border-accent rounded-md">
			{#if relationship.blocking}
				Blocked
			{:else}
				Block
			{/if}
		</button>
	</div>

	<div class="my-8 grid grid-cols-3 items-center justify-items-center">
		<span class="font-bold">Posts</span>
		<span class="font-bold">Following</span>
		<span class="font-bold">Followers</span>

		<span>{account.statuses_count}</span>
		<span>{account.following_count}</span>
		<span>{account.followers_count}</span>
	</div>

	<span class="my-4 text-center">
		<RenderedContent
			htmlContent={account.note}
			emojis={account.emojis}
		/>
	</span>

	{#if !isCondensed}
		<table class="w-full my-8">
			<thead class="">
				<tr>
					<th
						scope="col"
						class="px-4"
					></th>
					<th
						scope="col"
						colspan="3"
						class="px-4"
					></th>
				</tr>
			</thead>

			<tbody class="m-2">
				{#each account.fields as field}
					<tr>
						{#if field.verified}
							<td class="px-4 py-1 border border-accent w-1/3">
								{field.name}
								<span>(verified)</span>
							</td>
						{:else}
							<td class="px-4 py-1 border border-accent w-1/3">{field.name}</td>
						{/if}
						<td
							class="px-4 py-1 border border-accent overflow-ellipsis whitespace-nowrap overflow-hidden"
						>
							<RenderedContent
								htmlContent={field.value}
								emojis={account.emojis}
							/>
						</td>
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
					onOpen={(status) => {
						handleStatusOpen(status);
					}}
				/>
			{/each}
		</div>
	{/if}
</div>
