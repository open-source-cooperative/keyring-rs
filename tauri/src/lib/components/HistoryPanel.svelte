<script lang="ts">
	import type { HistoryEntry } from '../commands';
	import { Listgroup } from 'flowbite-svelte';
	import type { Readable } from 'svelte/store';

	let {
		history,
		selected = $bindable()
	}: {
		history: Readable<HistoryEntry[]>;
		selected: HistoryEntry | undefined;
	} = $props();

	interface ListItem {
		name: string;
		current: boolean;
		onclick: () => void;
	}

	let entries: ListItem[] = $derived(
		$history.map((entry) => {
			let name = `Entry ${entry.id}`;
			if (entry.is_specifier) {
				name += ` (${entry.service}, ${entry.user})`;
			}
			let current = entry.id === selected?.id;
			let onclick = () => {
				if (selected?.id === entry.id) {
					selected = undefined;
				} else {
					selected = entry;
				}
			};
			return { name, current, onclick } as ListItem;
		})
	);
</script>

{#if entries.length}
	<div class="w-full pr-4 pl-4">
		<Listgroup active items={entries} />
	</div>
{:else}
	<p class="text-center italic">No history</p>
{/if}
