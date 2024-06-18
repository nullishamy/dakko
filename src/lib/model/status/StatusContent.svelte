<script lang="ts">
  import * as api from '$lib/api';
  import RenderedContent from '$lib/generic/RenderedContent.svelte';
  import StatusAttachments from './StatusAttachments.svelte';

  export let status: api.Status;
  const { reblog } = status;

  let showSensitive = false;
  const spoilerText = (reblog ?? status).spoiler_text;
  const handleShowSensitive = () => {
    showSensitive = !showSensitive;
  };
</script>

{#if reblog}
  {#if reblog.spoiler_text && showSensitive}
    <div class="flex flex-col">
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
    <div class="text-left">
      <RenderedContent
        htmlContent={reblog.content}
        emojis={reblog.emojis}
      />
    </div>
  {:else if reblog.spoiler_text && !showSensitive}
    <div class="flex flex-col">
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
    <div class="text-left">
      <RenderedContent
        htmlContent={reblog.content}
        emojis={reblog.emojis}
      />
    </div>
  {/if}
{:else if status.spoiler_text && showSensitive}
  <div class="flex flex-col">
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
  <div class="text-left">
    <RenderedContent
      htmlContent={status.content}
      emojis={status.emojis}
    />
  </div>
{:else if status.spoiler_text && !showSensitive}
  <div class="flex flex-col">
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
  <div class="text-left">
    <RenderedContent
      htmlContent={status.content}
      emojis={status.emojis}
    />
  </div>
{/if}

{#if (spoilerText && showSensitive) || !spoilerText}
  <StatusAttachments {status} />
{/if}
