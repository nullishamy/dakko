<script lang="ts">
	import * as api from '$lib/api';

	export let htmlContent: string;
	export let emojis: api.Emoji[];

	function textNodesToSpan(elements: Element | ChildNode[], recursive: boolean) {
		recursive = recursive || true;
		if (!('length' in elements)) {
			// to array
			elements = [elements];
		}
		for (const node of elements) {
			if (node.nodeType === 3) {
				const span = document.createElement('span');
				span.classList.add('from-text');
				span.textContent = node.textContent;
				node.parentElement!.insertBefore(span, node);
				node.remove();
			} else if (recursive) {
				textNodesToSpan([...node.childNodes], recursive);
			}
		}
	}

	const parser = new DOMParser();
	const doc = parser.parseFromString(htmlContent, 'text/html');
	textNodesToSpan(doc.body, true);

	// Adapted from https://git.pleroma.social/pleroma/pleroma-fe/-/blob/develop/src/services/html_converter/utility.service.js?ref_type=heads#L53
	for (const node of doc.querySelectorAll('span.from-text')) {
		const parent = node.parentElement;
		// Skip `@` mentions, those use text nodes so get wrapped up as part of this
		if (parent?.nodeName == 'A') {
			continue;
		}

		const content = node.textContent ?? '';

		const nodes = [];
		let textBuffer = '';

		for (let i = 0; i < content.length; i++) {
			const char = content[i];
			if (char === ':') {
				const rest = content.slice(i + 1);
				const shortcode = rest.slice(0, rest.indexOf(':'));
				let emoji = emojis.find((e) => e.shortcode == shortcode);

				if (emoji) {
					const sp = doc.createElement('span');
					sp.textContent = textBuffer;
					nodes.push(sp);
					textBuffer = '';

					const img = doc.createElement('img');
					img.src = emoji.url;
					img.width = 25;
					img.height = 25;
					img.style.display = 'inline-block';
					nodes.push(img);

					i += emoji.shortcode.length + 1;
					emoji = undefined;
				} else {
					textBuffer += char;
				}
			} else {
				textBuffer += char;
			}
		}

		if (textBuffer) {
			const sp = doc.createElement('span');
			sp.textContent = textBuffer;
			nodes.push(sp);
		}

		node.innerHTML = '';
		for (const newNode of nodes) {
			node.appendChild(newNode);
		}
	}
</script>

<span class="block">{@html doc.body.innerHTML}</span>
