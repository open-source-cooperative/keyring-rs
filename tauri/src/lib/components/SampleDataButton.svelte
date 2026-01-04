<script lang="ts">
	import { createSampleEntries } from '$lib/sampleData';
	import { getAllEntries, type HistoryEntry } from '$lib/commands';
	import type { Writable } from 'svelte/store';
	import { Button, Alert } from 'flowbite-svelte';

	let {
		history,
		error = $bindable(),
		message = $bindable()
	}: {
		history: Writable<HistoryEntry[]>;
		error: string;
		message: string;
	} = $props();

	let createdCount = $state(0);

	function addSampleData() {
		error = '';
		message = '';
		createSampleEntries((result) => {
			if (result.count) {
				message = `${result.count} sample entries added.`;
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

<Button color="light" onclick={addSampleData}>Add Sample Data</Button>
