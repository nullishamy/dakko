<script
	lang="ts"
	generics="T, K extends keyof T"
>
	import { isBrowser, Virtual, type VirtualRange } from './virtual';
	import VirtualItem from './VirtualItem.svelte';
	import { createEventDispatcher, onDestroy, onMount } from 'svelte';

	/**
	 * Unique key for getting data from `data`
	 */
	export let key: K;
	/**
	 * Source for list
	 */
	export let data: T[];
	/**
	 * Count of rendered items
	 * @type {number}
	 */
	export let keeps = 30;
	/**
	 * Estimate size of each item, needs for smooth scrollbar
	 * @type {number}
	 */
	export let estimateSize = 500;
	/**
	 * Scroll direction
	 * @type {boolean}
	 */
	export let isHorizontal = false;
	/**
	 * scroll position start index
	 */
	export let start = 0;
	/**
	 * scroll position offset
	 */
	export let offset = 0;
	/**
	 * Let virtual list using global document to scroll through the list
	 * @type {boolean}
	 */
	export let pageMode = false;
	/**
	 * The threshold to emit `top` event, attention to multiple calls.
	 * @type {number}
	 */
	export let topThreshold = 0;
	/**
	 * The threshold to emit `bottom` event, attention to multiple calls.
	 * @type {number}
	 */
	export let bottomThreshold = 0;

	export let footerItemStyles = '';
	export let rootStyles = '';
	export let scrollWrapperStyles = '';

	let displayItems: T[] = [];
	let paddingStyle: string;
	let directionKey = isHorizontal ? 'scrollLeft' : 'scrollTop';
	let range = null;
	let virtual = new Virtual(
		{
			slotHeaderSize: 0,
			slotFooterSize: 0,
			keeps: keeps,
			estimateSize: estimateSize,
			buffer: Math.round(keeps / 3), // recommend for a third of keeps
			uniqueIds: getUniqueIdFromDataSources()
		},
		onRangeChanged
	);

	let root: Element;
	let shepherd: HTMLElement;
	const dispatch = createEventDispatcher();

	export function getSize(id: T[K]): number {
		const size = virtual.sizes.get(id);
		if (size === undefined) {
			throw new TypeError(`unknown id ${id}`);
		}
		return size;
	}

	export function getSizes(): number {
		return virtual.sizes.size;
	}

	export function getOffset(): number {
		if (pageMode && isBrowser()) {
			// @ts-expect-error FIXME: Find out how to make this safe
			return document.documentElement[directionKey] || document.body[directionKey];
		} else {
			// @ts-expect-error FIXME: Find out how to make this safe
			return root ? Math.ceil(root[directionKey]) : 0;
		}
	}

	/**
	 * @type {() => number}
	 */
	export function getClientSize() {
		const key = isHorizontal ? 'clientWidth' : 'clientHeight';
		if (pageMode && isBrowser()) {
			return document.documentElement[key] || document.body[key];
		} else {
			return root ? Math.ceil(root[key]) : 0;
		}
	}

	/**
	 * @type {() => number}
	 */
	export function getScrollSize() {
		const key = isHorizontal ? 'scrollWidth' : 'scrollHeight';
		if (pageMode && isBrowser()) {
			return document.documentElement[key] || document.body[key];
		} else {
			return root ? Math.ceil(root[key]) : 0;
		}
	}

	export function updatePageModeFront(): void {
		if (root && isBrowser()) {
			const rect = root.getBoundingClientRect();
			const { defaultView } = root.ownerDocument;
			if (!defaultView) {
				throw new TypeError(`no defaultView for root`);
			}

			const offsetFront = isHorizontal
				? rect.left + defaultView.pageXOffset
				: rect.top + defaultView.pageYOffset;
			virtual.updateParam('slotHeaderSize', offsetFront);
		}
	}

	export function scrollToOffset(offset: number): void {
		if (!isBrowser()) return;
		if (pageMode) {
			// @ts-expect-error FIXME: Find out how to make this safe
			document.body[directionKey] = offset;
			// @ts-expect-error FIXME: Find out how to make this safe
			document.documentElement[directionKey] = offset;
		} else if (root) {
			// @ts-expect-error FIXME: Find out how to make this safe
			root[directionKey] = offset;
		}
	}

	export function scrollToIndex(index: number): void {
		if (index >= data.length - 1) {
			scrollToBottom();
		} else {
			const offset = virtual.getOffset(index);
			scrollToOffset(offset);
		}
	}

	export function scrollToBottom(): void {
		if (shepherd) {
			const offset = shepherd[isHorizontal ? 'offsetLeft' : 'offsetTop'];
			scrollToOffset(offset);

			// check if it's really scrolled to the bottom
			// maybe list doesn't render and calculate to last range,
			// so we need retry in next event loop until it really at bottom
			setTimeout(() => {
				if (getOffset() + getClientSize() + 1 < getScrollSize()) {
					scrollToBottom();
				}
			}, 3);
		}
	}

	onMount(() => {
		if (start) {
			scrollToIndex(start);
		} else if (offset) {
			scrollToOffset(offset);
		}

		if (pageMode) {
			updatePageModeFront();

			document.addEventListener('scroll', onScroll, {
				passive: false
			});
		}
	});

	onDestroy(() => {
		virtual.destroy();
		if (pageMode && isBrowser()) {
			document.removeEventListener('scroll', onScroll);
		}
	});

	function getUniqueIdFromDataSources() {
		return data.map((dataSource) => dataSource[key]);
	}

	function onItemResized(event: CustomEvent<{ id: T[K]; size: number; type: string }>) {
		const { id, size, type } = event.detail;
		if (type === 'item') {
			virtual.saveSize(id, size);
		} else if (type === 'slot') {
			if (id === 'header') {
				virtual.updateParam('slotHeaderSize', size);
			} else if (id === 'footer') {
				virtual.updateParam('slotFooterSize', size);
			}

			// virtual.handleSlotSizeChange()
		}
	}

	function onRangeChanged(range_: VirtualRange) {
		range = range_;
		paddingStyle = paddingStyle = isHorizontal
			? `0px ${range.padBehind}px 0px ${range.padFront}px`
			: `${range.padFront}px 0px ${range.padBehind}px`;
		displayItems = data.slice(range.start, range.end + 1);
	}

	function onScroll(event: unknown) {
		const offset = getOffset();
		const clientSize = getClientSize();
		const scrollSize = getScrollSize();

		// iOS scroll-spring-back behavior will make direction mistake
		if (offset < 0 || offset + clientSize > scrollSize || !scrollSize) {
			return;
		}

		virtual.handleScroll(offset);
		emitEvent(offset, clientSize, scrollSize, event);
	}

	function emitEvent(offset: number, clientSize: number, scrollSize: number, event: unknown) {
		dispatch('scroll', { event, range: virtual.getRange() });

		if (virtual.isFront() && !!data.length && offset - topThreshold <= 0) {
			dispatch('top');
		} else if (virtual.isBehind() && offset + clientSize + bottomThreshold >= scrollSize) {
			dispatch('bottom');
		}
	}

	$: scrollToOffset(offset);
	$: scrollToIndex(start);
	$: handleKeepsChange(keeps);

	function handleKeepsChange(keeps: number) {
		virtual.updateParam('keeps', keeps);
		virtual.handleSlotSizeChange();
	}

	$: handleDataSourcesChange(data);

	async function handleDataSourcesChange(data: unknown) {
		virtual.updateParam('uniqueIds', getUniqueIdFromDataSources());
		virtual.handleDataSourcesChange();
	}
</script>

<div
	bind:this={root}
	on:scroll={onScroll}
	style="overflow-y: auto"
	class={rootStyles}
>
	{#if $$slots.header}
		<VirtualItem
			on:resize={onItemResized}
			type="slot"
			uniqueKey="header"
		>
			<slot name="header" />
		</VirtualItem>
	{/if}
	<div
		style="padding: {paddingStyle}"
		class={scrollWrapperStyles}
	>
		{#each displayItems as dataItem, dataIndex (dataItem[key])}
			<VirtualItem
				on:resize={onItemResized}
				uniqueKey={dataItem[key]}
				horizontal={isHorizontal}
				type="item"
			>
				<slot
					data={dataItem}
					index={dataIndex}
				/>
			</VirtualItem>
		{/each}
	</div>
	{#if $$slots.footer}
		<VirtualItem
			type="slot"
			uniqueKey="footer"
			styles={footerItemStyles}
		>
			<slot name="footer" />
		</VirtualItem>
	{/if}
	<div
		bind:this={shepherd}
		class="shepherd"
		style="width: {isHorizontal ? '0px' : '100%'};height: {isHorizontal ? '100%' : '0px'}"
	></div>
</div>
