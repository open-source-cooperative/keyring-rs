<script lang="ts">
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { ChevronDownOutline } from 'flowbite-svelte-icons';
	import { platform } from '@tauri-apps/plugin-os';
	import { getStoreInfo, type HistoryEntry, releaseStore, useNamedStore, type VoidResult } from '$lib/commands';
	import type { Writable } from 'svelte/store';

	let {
		history,
		selected = $bindable(),
		error = $bindable(),
		message = $bindable(),
	}: {
		history: Writable<HistoryEntry[]>;
		selected: string | undefined;
		error: string;
		message: string;
	} = $props();

	let availableStores = ['none', 'sample'];
	switch (platform()) {
		case 'android':
			availableStores.push('android');
			break;
		case 'ios':
			availableStores.push('protected');
			break;
		case 'linux':
			availableStores.push('keyutils', 'secret-service');
			break;
		case 'macos':
			availableStores.push('keychain');
			break;
		case 'windows':
			availableStores.push('windows');
			break;
		default:
			availableStores.push('secret-service');
			break;
	}
	let chosenStore = $state('none');

	function setStore(name: string) {
		error = '';
		message = '';
		if (chosenStore == name) {
			return;
		}
		const handler = (result: VoidResult) => {
			if (result.error) {
				error = result.error;
			} else {
				chosenStore = name;
				history.set([]);
				selected = undefined;
				getStoreInfo((s) => {
					message = s;
				})
			}
		};
		switch (name) {
			case 'none':
				releaseStore(handler);
				break;
			default:
				useNamedStore(name, handler);
				break;
		}
	}

	let storeLabels: Record<string, string> = {
		sample: 'Keyring Core Sample',
		android: 'Android Shared Preferences',
		protected: 'iOS Protected Data',
		keychain: 'macOS Keychain',
		keyutils: 'Linux Keyutils',
		'secret-service': 'Secret Service',
		windows: 'Windows Credential Manager'
	};

	function buttonName(): string {
		if (chosenStore === 'none') {
			return 'Choose a credential store...';
		} else {
			return storeLabels[chosenStore];
		}
	}

	function choiceName(name: string): string {
		if (name === 'none') {
			return 'Release the current store';
		} else {
			return storeLabels[name];
		}
	}

	let isOpen = $state(false);
</script>

<Button color="light" onclick={() => (isOpen = true)}>
	{buttonName()}
	<ChevronDownOutline class="ms-0 w-6 text-black" />
</Button>
<Dropdown bind:isOpen simple>
	{#each availableStores as choice}
		{#if chosenStore !== choice}
			<DropdownItem
				onclick={() => {
					setStore(choice);
					isOpen = false;
				}}
			>
				{choiceName(choice)}
			</DropdownItem>
		{/if}
	{/each}
</Dropdown>
