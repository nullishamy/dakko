<script lang="ts">
	import { errors } from "./error-store";

  export let message: string
  errors.set([...$errors, message])
</script>
<h1 class="text-2xl">An error has occurred</h1>
<div class="flex flex-col gap-4">

{#each $errors as error}
  <span class="text-red">{error}</span>
{/each}
</div>
