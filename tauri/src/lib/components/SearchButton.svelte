<script lang="ts">
	import { getAllEntries, searchAll, type HistoryEntry } from '$lib/commands';
	import type { Writable } from 'svelte/store';
	import { Button, Alert } from 'flowbite-svelte';

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
			if (result.value) {
				message = `${result.value} entries found.`;
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
