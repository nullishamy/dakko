<script lang="ts">
	import type { Emoji } from './types';

	export let htmlContent: string;
	export let emojis: Emoji[];

	function doSubOnContent(content: string, doc: Document): Node[] {
    const nodes = []
		let textBuffer = '';

		for (let i = 0; i < content.length; i++) {
			const char = content[i];
			if (char === ':') {
				const next = content.slice(i + 1);

				let found = undefined;
				for (const emoji of emojis) {
					if (next.slice(0, emoji.shortcode.length + 1) === emoji.shortcode + ':') {
						found = emoji;
						break;
					}
				}

				if (found) {
					const sp = doc.createElement('span');
					sp.textContent = textBuffer;
					nodes.push(sp)
					textBuffer = '';

					const img = doc.createElement('img');
          img.innerHTML = `<img style="display: inline-block;" height="20" width="20" src="${found.url}" />`
					nodes.push(img)

					i += found.shortcode.length + 1;
          found = undefined
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
			nodes.push(sp)
		}

    return nodes
	}

	const parser = new DOMParser();
	const parsedDoc = parser.parseFromString(htmlContent, 'text/html');

  const doc = new Document()

  let html = document.createElement('html');
  html.appendChild(document.createElement('body'))
  doc.appendChild(html);

	const body = parsedDoc.body;
	const walker = parsedDoc.createTreeWalker(body, NodeFilter.SHOW_TEXT);

	while (walker.nextNode()) {
		const node = walker.currentNode;
		const content = node.textContent;

		if (!content) {
			throw new TypeError('no content in text node');
		}

    console.log(content)

		const nodes = doSubOnContent(content, doc);
    doc.body.append(...nodes)
	}
  console.log(doc)
</script>

<span class="block">{@html doc.body.innerHTML}</span>
