import { invoke } from '@tauri-apps/api/core';

export interface HistoryEntry {
	id: string;
	is_specifier: boolean;
	service?: string;
	user?: string;
}

export interface VoidResult {
	error?: string;
}

export interface StringResult {
	value?: string;
	error?: string;
}

export interface MapResult {
	value?: { [key: string]: string };
	error?: string;
}

export interface EntryResult {
	value?: HistoryEntry;
	error?: string;
}

export interface HistoryResult {
	value?: HistoryEntry[];
	error?: string;
}

export interface CountResult {
	value?: number;
	error?: string;
}

export function useNamedStore(name: string, handler: (result: VoidResult) => void) {
	invoke('use_named_store', { name })
		.then(() => handler({} as VoidResult))
		.catch((error) => handler({ error } as VoidResult));
}

export function releaseStore(handler: (result: VoidResult) => void) {
	invoke('release_store')
		.then(() => handler({} as VoidResult))
		.catch((error) => handler({ error } as VoidResult));
}

export function getEntry(id: string, handler: (result: EntryResult) => void) {
	invoke('get_entry', { id })
		.then((entry) => handler({ value: entry } as EntryResult))
		.catch((error) => handler({ error } as EntryResult));
}

export function getAllEntries(handler: (result: HistoryResult) => void) {
	invoke('get_all_entries')
		.then((entries) => handler({ value: entries } as HistoryResult))
		.catch((error) => handler({ error } as HistoryResult));
}

export function removeEntry(id: string, handler: (result: VoidResult) => void) {
	invoke('remove_entry', { id })
		.then(() => handler({} as VoidResult))
		.catch((error) => handler({ error } as VoidResult));
}

export function entryNew(service: string, user: string, handler: (result: EntryResult) => void) {
	invoke('entry_new', { service, user })
		.then((entry) => handler({ value: entry } as EntryResult))
		.catch((error) => handler({ error } as EntryResult));
}

export function entryGetValue(id: string, handler: (result: StringResult) => void) {
	invoke('entry_get_value', { id })
		.then((value) => handler({ value } as StringResult))
		.catch((error) => handler({ error } as StringResult));
}

export function entrySetValue(id: string, value: string, handler: (result: VoidResult) => void) {
	invoke('entry_set_value', { id, value })
		.then(() => handler({} as VoidResult))
		.catch((error) => handler({ error } as VoidResult));
}

export function entryGetAttributes(id: string, handler: (result: MapResult) => void) {
	invoke('entry_get_attributes', { id })
		.then((attributes) => handler({ value: attributes } as MapResult))
		.catch((error) => handler({ error } as MapResult));
}

export function entryUpdateAttributes(
	id: string,
	attributes: { [key: string]: string },
	handler: (result: VoidResult) => void
) {
	invoke('entry_update_attributes', { id, attributes })
		.then(() => handler({} as VoidResult))
		.catch((error) => handler({ error } as VoidResult));
}

export function entryDeleteValue(id: string, handler: (result: VoidResult) => void) {
	invoke('entry_delete_value', { id })
		.then(() => handler({} as VoidResult))
		.catch((error) => handler({ error } as VoidResult));
}

export function searchAll(handler: (result: CountResult) => void) {
	invoke('search_all')
		.then((count) => handler({ value: count } as CountResult))
		.catch((error) => handler({ error } as CountResult));
}
