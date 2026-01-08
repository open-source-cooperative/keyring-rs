<script lang="ts">
	import { getAllEntries, type HistoryEntry, searchAll } from '$lib/commands';
	import type { Writable } from 'svelte/store';
	import { Button } from 'flowbite-svelte';

	let {
		history,
		selected = $bindable(),
		error = $bindable(),
		message = $bindable()
	}: {
		history: Writable<HistoryEntry[]>;
		selected: string | undefined;
		error: string;
		message: string;
	} = $props();

	function search() {
		error = '';
		message = '';
		searchAll((result) => {
			if (result.value !== undefined) {
				if (result.value === 1) {
					message = `${result.value} entry found.`;
				} else {
					message = `${result.value} entries found.`;
				}
			}
			if (result.error) {
				error = result.error;
			}
			getAllEntries((result2) => {
				if (result2.value) {
					history.set(result2.value);
				}
				if (result2.error) {
					const additional = `Couldn't load new entries: ${result2.error}`;
					if (error) {
						error += `; ${additional}`;
					} else {
						error = additional;
					}
				}
			});
		});
	}
</script>

<Button color="light" onclick={search}>Search All</Button>
