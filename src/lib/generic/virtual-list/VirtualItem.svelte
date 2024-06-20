<script lang="ts" generics="K">
	import { afterUpdate, createEventDispatcher, onDestroy, onMount } from 'svelte';

	export let horizontal = false;
	export let uniqueKey: K;
	export let type = 'item';
  export let styles = ''

	let resizeObserver: ResizeObserver | undefined;
	let itemDiv: HTMLDivElement;
	let previousSize: number;

	const dispatch = createEventDispatcher();
	const shapeKey = horizontal ? 'offsetWidth' : 'offsetHeight';

	onMount(() => {
    resizeObserver = new ResizeObserver(dispatchSizeChange);
    resizeObserver.observe(itemDiv);
	});
	afterUpdate(dispatchSizeChange);
	onDestroy(() => {
		if (resizeObserver) {
			resizeObserver.disconnect();
			resizeObserver = undefined;
		}
	});

	function dispatchSizeChange() {
		const size = itemDiv ? itemDiv[shapeKey] : 0;
		if (size === previousSize) return;
		previousSize = size;
		dispatch('resize', { id: uniqueKey, size, type });
	}
</script>

<div
	bind:this={itemDiv}
	class={styles}
>
	<slot />
</div>
