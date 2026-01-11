<script lang="ts">
	import { Alert } from 'flowbite-svelte';
	import SelectStoreButton from '$lib/components/SelectStoreButton.svelte';
	import SampleDataButton from '$lib/components/SampleDataButton.svelte';
	import type { Writable } from 'svelte/store';
	import type { HistoryEntry } from '$lib/commands';
	import NewEntryButton from '$lib/components/NewEntryButton.svelte';
	import SearchButton from '$lib/components/SearchButton.svelte';

	let {
		history,
		selected = $bindable()
	}: {
		history: Writable<HistoryEntry[]>;
		selected: HistoryEntry | undefined;
	} = $props();

	let error = $state('');
	let message = $state('');
</script>

<div class="w-full p-4">
	<div class="flex items-center justify-center gap-2">
		<SelectStoreButton {history} bind:selected bind:error bind:message />
		<SampleDataButton {history} bind:error bind:message />
		<NewEntryButton {history} bind:selected bind:error />
		<SearchButton {history} bind:selected bind:error bind:message />
	</div>
	{#if message}
		<div class="w-full p-4 pb-0">
			<Alert color="green" onclick={() => (message = '')} dismissable>{message}</Alert>
		</div>
	{/if}
	{#if error}
		<div class="w-full p-4 pb-0">
			<Alert color="red" onclick={() => (error = '')} dismissable>{error}</Alert>
		</div>
	{/if}
</div>
