<script lang='ts'>
	import { getContext } from "svelte";
	import { type Context, mainContext } from "./context";
	import Timeline from "./Timeline.svelte";
	import AccountView from "./AccountView.svelte";

  const { content } = getContext<Context>(mainContext)
</script>
{#if $content?.type == 'user'}
  <AccountView account={$content.account}/>
{:else if $content?.type == 'timeline'}
  <Timeline timeline={$content.timeline} />
{:else}
  <span>Loading...</span>
{/if}