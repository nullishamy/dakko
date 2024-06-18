<script lang="ts">
	import * as api from '$lib/api';
	import { mainContext, type MainContext } from '$lib/context';
	import CompositionArea from '$lib/generic/CompositionArea.svelte';
	import { fullyQualifiedAccount } from '$lib/utils';
	import Icon from '@iconify/svelte';
	import { getContext } from 'svelte';
	import AccountView from './AccountView.svelte';
	import StatusControls from './status/StatusControls.svelte';
	import StatusContent from './status/StatusContent.svelte';
	import RenderedContent from '../generic/RenderedContent.svelte';
	import StatusButtons from './status/StatusButtons.svelte';

	export let onOpen: (status: api.Status) => void;
	export let highlighted: boolean = false;

	export let status: api.Status;
	export let replyTo: api.Account | undefined = undefined;

	const reblog = status.reblog;
	const { content } = getContext<MainContext>(mainContext);

	let replyOpen = false;
	let quoteOpen = false;

	let openedUser: api.Account | undefined;
	const onUserSelect = (user: api.Account) => {
		openedUser = openedUser?.id == user.id ? undefined : user;
	};

	const prefillReply = (val: 'quote' | 'reply') => {
		const header = fullyQualifiedAccount((reblog ?? status).account) + ' ';
		let footer = '';
		if (val === 'quote') {
			footer = `\n\nRE: ${status.url}`;
		}
		return header + footer;
	};

	const handlePost = async (val: 'quote' | 'post', data: api.StatusContent) => {
		if (val === 'quote') {
			await api.postStatus({
				...data,
				quoting: status.id
			});
			quoteOpen = false;
			return;
		}
		await api.replyToStatus(status.id, data);
		replyOpen = false;
	};

  const pronouns = (reblog ?? status).account.fields.find(p => p.name.toLowerCase() === 'pronouns')
</script>

<div class={highlighted ? 'border-2 border-text rounded-md p-2' : 'p-2'}>
	{#if reblog}
		<div class="ml-10 mb-1 flex flex-row items-center gap-2 h-6 font-bold">
			<!-- svelte-ignore a11y-missing-attribute -->
			<img
				src={status.account.avatar}
				class="h-full rounded-lg"
			/>
			<RenderedContent
				htmlContent={status.account.display_name}
				emojis={status.account.emojis}
			/>
			<button
				class="text-blue font-normal"
				on:click={() => onUserSelect(status.account)}
			>
				@{status.account.acct}
			</button>
			<span class="font-normal text-subtext0">boosted</span>
		</div>
	{/if}

	<div class="relative flex flex-row gap-2">
		<!-- Left side-->
		<div class="w-16">
			<!-- svelte-ignore a11y-missing-attribute -->
			{#if reblog}
				<img
					src={reblog.account.avatar}
					class="w-full rounded-md"
				/>
			{:else}
				<img
					src={status.account.avatar}
					class="w-full rounded-md"
				/>
			{/if}
		</div>
		<!-- Right side-->
		<div class="flex-1">
			{#if openedUser}
				<div class="bg-mantle rounded-md z-10 min-w-full border border-accent mb-4">
					<AccountView
						account={openedUser}
						isCondensed={true}
						onClose={() => (openedUser = undefined)}
						onOpen={(account) => {
							content.set({
								type: 'user',
								account
							});
						}}
					/>
				</div>
			{/if}

			{#if reblog}
				<div class="flex flex-row items-center gap-2 font-bold">
          <span class="min-w-max">
            <RenderedContent
              htmlContent={reblog.account.display_name}
              emojis={reblog.account.emojis}
            />
          </span>
					<button
						class="text-blue font-normal"
						on:click={() => onUserSelect(reblog.account)}
					>
						@{reblog.account.acct}
					</button>

          {#if pronouns}
            <span class="font-normal">
              <RenderedContent 
                htmlContent={`(${pronouns.value.slice(0, 30)})`}
                emojis={reblog.account.emojis}
              />
            </span>
          {/if}

					<div class="flex-grow" />

					<StatusControls
						{status}
						onStatusExpand={onOpen}
					/>
				</div>

				<div class="mt-1">
					<StatusContent status={reblog} />
				</div>
			{:else}
				<div class="flex flex-row items-center gap-2 font-bold">
          <span>
            <RenderedContent
              htmlContent={status.account.display_name}
              emojis={status.account.emojis}
            />
          </span>
					<button
						class="text-blue font-normal"
						on:click={() => onUserSelect(status.account)}
					>
						@{status.account.acct}
					</button>

          {#if pronouns}
            <span class="font-normal">
              <RenderedContent 
                htmlContent={`(${pronouns.value.slice(0, 30)})`}
                emojis={status.account.emojis}
              />
            </span>
          {/if}

					<div class="flex-grow" />

					<StatusControls
						{status}
						onStatusExpand={onOpen}
					/>
				</div>
				{#if replyTo}
					<p class="text-subtext0 text-sm flex flex-row items-center gap-2">
						<Icon
							icon="material-symbols:reply"
							flip="horizontal"
						/>
						Reply to
						<span class="text-blue gap-0">
							{fullyQualifiedAccount(replyTo)}
						</span>
					</p>
				{/if}

				<div class="mt-1">
					<StatusContent {status} />
				</div>
			{/if}

			<StatusButtons
				{status}
				bind:quoteOpen
				bind:replyOpen
			/>

			{#if replyOpen}
				<div class="p-2 my-2 bg-mantle rounded-md">
					<CompositionArea
						onPost={(d) => handlePost('post', d)}
						content={prefillReply('reply')}
						cw={status.sensitive ? `re: ${status.spoiler_text}` : undefined}
					/>
				</div>
			{/if}

			{#if quoteOpen}
				<div class="p-2 my-2 bg-mantle rounded-md">
					<CompositionArea
						onPost={(d) => handlePost('quote', d)}
						content={prefillReply('quote')}
						cw={status.sensitive ? `re: ${status.spoiler_text}` : undefined}
					/>
				</div>
			{/if}
		</div>
	</div>
</div>
