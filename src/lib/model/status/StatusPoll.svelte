<script lang="ts">
	import * as api from '$lib/api';
	import { formatDistanceStrict } from 'date-fns';

	export let poll: api.Poll;
	function expirationString() {
    if (!poll.expires_at) {
      if (poll.expired) {
        return 'Poll expired long ago'
      } else {
        return 'Poll never expires'
      }
    }

		const now = new Date();
		const date = new Date(poll.expires_at);
		if (poll.expired) {
			return `Poll ended ${formatDistanceStrict(date, now, { addSuffix: true })}`;
		} else {
			return `Poll ends ${formatDistanceStrict(date, now, { addSuffix: true })}`;
		}
	}

	let votedForOptions: Set<number> = new Set();

	async function submitVotes() {
    poll = await api.voteForPoll(poll, [...votedForOptions])
	}
</script>

<div class="m-auto mt-2 flex flex-col gap-2">
	{#if poll.voted || poll.expired}
		{#each poll.options as option}
			{@const percentage = Math.round((option.votes_count / poll.votes_count) * 100)}
			<div class="flex flex-row relative bg-mantle border border-accent p-1 pl-2 rounded-lg mr-16">
				<span class="flex z-10">
					<span class="w-4">{percentage}%</span>
					<div class="w-8" />
					<span class="">{option.title}</span>
				</span>
				<div
					style={`width: ${percentage}%;`}
					class={`bg-accent opacity-35 absolute h-full top-0 left-0 rounded-lg`}
				></div>
			</div>
		{/each}
	{:else}
		{#each poll.options as option, index}
			<div
				class="flex flex-row items-center gap-2 bg-mantle border border-accent p-1 pl-2 rounded-lg mr-16"
			>
				<input
					class="ml-1"
					type={poll.multiple ? 'checkbox' : 'radio'}
					name="poll"
					on:click={() => {
            if (!poll.multiple) {
              votedForOptions.clear()
            }
						votedForOptions.add(index);
					}}
				/>
				<span class="">{option.title}</span>
			</div>
		{/each}
	{/if}

	{#if poll.voted || poll.expired}
		<span class="text-subtext0">
			{poll.voters_count} people voted ({poll.votes_count} votes) | {expirationString()}
		</span>
	{:else}
		<div class="flex flex-row items-center gap-4">
			<button
				class="px-4 py-0.5 bg-mantle rounded-md border border-accent"
				on:click={submitVotes}
			>
				Vote
			</button>
			<span class="text-subtext0">
				{poll.voters_count} people voted ({poll.votes_count} votes) | {expirationString()}
			</span>
		</div>
	{/if}
</div>
