<script lang="ts">
	import RenderedContent from './RenderedContent.svelte';
	import type { Account } from './types';

	export let account: Account;
</script>

<div class="w-full p-8 flex flex-col">
	<div class="flex flex-row gap-4">
		<img
			src={account.avatar}
			alt="Avatar for {account.display_name}"
			width="75"
			height="75"
			class="bg-mantle rounded-md"
		/>

		<div class="flex flex-col gap-2">
			<span class="text-lg">{account.display_name}</span>
			<span class="text-lg">@{account.username}</span>
		</div>
	</div>

  <span class="justify-center my-4"><RenderedContent htmlContent={account.note} emojis={account.emojis}/></span>

	<div class="mt-8 grid grid-cols-3 items-center justify-items-center">
		<span class="font-bold">Posts</span>
		<span class="font-bold">Following</span>
		<span class="font-bold">Followers</span>

		<span>{account.statuses_count}</span>
		<span>{account.following_count}</span>
		<span>{account.followers_count}</span>
	</div>

	<table class="w-full mt-8">
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
						<td class="px-4 py-1 border border-pink w-1/3">{field.name} <span>(verified)</span></td>
					{:else}
						<td class="px-4 py-1 border border-pink w-1/3">{field.name}</td>
					{/if}
					<td class="px-4 py-1 border border-pink">{@html field.value}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
