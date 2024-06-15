<script lang="ts">
  import { getContext } from 'svelte';
  import { type Context, mainContext } from './context';
	import CompositionArea from './CompositionArea.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

  const { account, content } = getContext<Context>(mainContext);
  let composeOpen = false

  const toggleCompose = () => {
    composeOpen = !composeOpen
  }

  const postStatus = (status: { content: string, cw: string | undefined }) => {
    invoke('post_status', {
      status
    }).then(() => {
      toggleCompose()
    })
  }

  const openOwnAccount = () => {
    content.set({
      type: 'user',
      account: $account
    })
  }
</script>

<div class="flex flex-col gap-4 items-center justify-items-center h-full">
  <button on:click={openOwnAccount} class="flex flex-col items-center mb-4">
    <img
      src={$account?.avatar}
      alt="Avatar for {$account?.display_name}"
      width="100"
      height="100"
      class="bg-mantle rounded-md"
    />
    <span class="text-sm">{$account?.display_name ?? 'Your Display Name'}</span>
    <span class="text-sm">@{$account?.username ?? 'your-username'}</span>
  </button>

  <div class="relative">
    <button on:click={toggleCompose}>Compose</button>
    {#if composeOpen}
      <div class="absolute bg-mantle drop-shadow-md p-2">
        <CompositionArea onPost={postStatus}/>
      </div>
    {/if}
  </div>

  <span>Announcements</span>
  <span>Follow requests</span>
  <span>About</span>

  <hr class="bg-pink h-0.5 w-4/5" />

  <div class="flex-grow" />

  <span class="">Cog</span>
</div>
