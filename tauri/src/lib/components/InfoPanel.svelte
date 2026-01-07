<script lang="ts">
	import {
		entryDeleteValue,
		entryGetAttributes,
		entryGetValue,
		entrySetValue,
		type HistoryEntry
	} from '$lib/commands';
	import { Alert, Button, Heading, Input, Label, List, Li, Modal, P } from 'flowbite-svelte';

	let { selected }: { selected: HistoryEntry | undefined } = $props();

	let name = $derived(
		selected
			? `Entry ${selected.id} (${selected.is_specifier ? 'specifier' : 'wrapper'})`
			: 'No entry selected'
	);
	let service = $derived(selected?.is_specifier ? selected.service : '');
	let user = $derived(selected?.is_specifier ? selected.user : '');
	let value = $derived(selected ? '' : '');
	let valueError = $derived(selected ? '' : '');
	let dataModal = $state(false);

	function retrieveValue() {
		valueError = '';
		if (!selected) {
			value = '';
			return;
		}
		entryGetValue(selected.id, (result) => {
			if (result.error) {
				valueError = `Retrieve of data failed: ${result.error}`;
				value = '';
			}
			if (result.value) {
				value = result.value;
			}
		});
	}

	function setPassword(password: string) {
		valueError = '';
		if (!selected) {
			return;
		}
		const newValue = `UTF8:${password}`;
		entrySetValue(selected.id, newValue, (result) => {
			if (result.error) {
				valueError = `Set of password failed: ${result.error}`;
			} else {
				value = newValue;
			}
		});
	}

	function deleteValue() {
		valueError = '';
		if (!selected) {
			return;
		}
		entryDeleteValue(selected.id, (result) => {
			if (result.error) {
				valueError = `Delete of data failed: ${result.error}`;
			} else {
				value = '';
			}
		});
	}

	function dataModalAction({ action, data }: { action: string; data: FormData }) {
		if (action === 'set-password') {
			setPassword(data.get('password') as string);
		} else if (action === 'delete-value') {
			deleteValue();
		}
		dataModal = false;
	}

	let attributes: [string, string][] | undefined = $derived(selected ? undefined : undefined);
	let attributeError = $derived(selected ? '' : '');

	function retrieveAttributes() {
		attributeError = '';
		if (!selected) {
			attributes = undefined;
			return;
		}
		entryGetAttributes(selected.id, (result) => {
			if (result.error) {
				attributeError = `Retrieve of attributes failed: ${result.error}`;
				attributes = undefined;
			}
			if (result.value) {
				attributes = Object.entries(result.value as Record<string, string>);
				attributes.sort((a, b) => a[0].localeCompare(b[0]));
			}
		});
	}
</script>

<div class="flex justify-center">
	<Heading tag="h5">{name}</Heading>
</div>
{#if selected}
	{#if selected.is_specifier}
		<div class="px-2">
			<P class="font-normal"><span class="font-semibold">Service:</span> {service}</P>
			<P class="font-normal"><span class="font-semibold">User:</span> {user}</P>
		</div>
	{/if}
	<div class="m-2 space-y-4 border-t border-gray-200 px-2">
		<div class="align-center mt-2 flex gap-2">
			<Button onclick={retrieveValue} color="light">Retrieve Data</Button>
			<Button onclick={() => (dataModal = true)} color="light">Update Data</Button>
		</div>
		<Modal title="Update Data" form bind:open={dataModal} onaction={dataModalAction} class="w-100">
			<div>
				<Label for="password">Password:</Label>
				<Input type="text" name="password" />
				<Button type="submit" value="set-password" color="green">Set Password</Button>
			</div>
			<Button type="submit" value="delete-value" color="red">Delete Credential</Button>
			<Button outline type="submit" value="cancel">Cancel</Button>
		</Modal>
		{#if value}
			{#if value.startsWith('UTF8:')}
				<P class="font-normal"><span class="font-semibold">Password:</span> {value.substring(5)}</P>
			{:else}
				<P class="font-normal"><span class="font-semibold">Secret:</span> {value.substring(4)}</P>
			{/if}
		{/if}
		{#if valueError}
			<Alert color="red" dismissable>{valueError}</Alert>
		{/if}
	</div>
	<div class="m-2 space-y-4 border-t border-gray-200 px-2">
		<div class="mt-2">
			<Button onclick={retrieveAttributes} color="light">Retrieve Attributes</Button>
		</div>
		{#if attributes}
			{#if attributes.length === 0}
				<P class="italic">No attributes</P>
			{:else}
				<List class="list-disc">
					{#each attributes as [key, value]}
						<Li><span class="font-semibold">{key}:</span> {value}</Li>
					{/each}
				</List>
			{/if}
		{/if}
		{#if attributeError}
			<Alert color="red" dismissable>{attributeError}</Alert>
		{/if}
	</div>
{/if}
