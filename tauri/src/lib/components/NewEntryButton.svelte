<script lang="ts">
	import { Input, Label, Modal } from 'flowbite-svelte';
	import { entryNew, getAllEntries, type HistoryEntry } from '$lib/commands';
	import type { Writable } from 'svelte/store';
	import { Button } from 'flowbite-svelte';

	let {
		history,
		selected = $bindable(),
		error = $bindable()
	}: {
		history: Writable<HistoryEntry[]>;
		selected: HistoryEntry | undefined;
		error: string;
	} = $props();

	function newEntry(service: string, user: string) {
		error = '';
		entryNew(service, user, (result) => {
			if (result.error) {
				error = result.error;
				return;
			}
			if (result.value) {
				getAllEntries((result2) => {
					if (result2.error) {
						error = result2.error;
					}
					if (result2.value) {
						history.set(result2.value);
						selected = result.value;
					}
				});
			}
		});
	}

	let entryModal = $state(false);
	function entryModalAction({ action, data }: { action: string; data: FormData }) {
		if (action === 'create') {
			newEntry(data.get('service') as string, data.get('user') as string);
		}
		entryModal = false;
	}
</script>

<Button color="light" onclick={() => (entryModal = true)}>New Entry</Button>
<Modal title="New Entry" form bind:open={entryModal} onaction={entryModalAction} class="w-[400px]">
	<div>
		<Label for="service">Service:</Label>
		<Input type="text" name="service" />
	</div>
	<div>
		<Label for="user">User:</Label>
		<Input type="text" name="user" />
	</div>
	<Button type="submit" value="create" color="green">Create Entry</Button>
	<Button outline type="submit" value="cancel">Cancel</Button>
</Modal>
