import {
	entryDeleteValue,
	entryGetValue,
	entryNew,
	entrySetValue,
	type VoidResult
} from '$lib/commands';

interface SampleEntry {
	service: string;
	user: string;
	password?: string;
	secret?: string;
}
const sampleDataEntries: SampleEntry[] = [
	{ service: 'dummy', user: 'dummy1', password: 'dummy1-password' },
	{ service: 'dummy', user: 'dummy2' },
	{ service: 'dummy', user: 'dummy3', secret: 'b5eb2d3dab2cc28add' }
];

function createSampleEntry(entry: SampleEntry, handler: (result: VoidResult) => void) {
	entryNew(entry.service, entry.user, (result) => {
		if (result.error) {
			handler({ error: result.error } as VoidResult);
		} else {
			// created OK
			if (!entry.password && !entry.secret) {
				handler({});
				return;
			}
			// don't set entry or password if it already exists
			entryGetValue(result.value!.id, (result2) => {
				if (result2.value) {
					handler({});
					return;
				}
				// ignore errors, they will come back on set if real
				let value = entry.password ? `UTF8:${entry.password}` : `HEX:${entry.secret ?? ''}`;
				entrySetValue(result.value!.id, value, (result2) => {
					if (result2.error) {
						handler({ error: result2.error } as VoidResult);
					} else {
						handler({});
					}
				});
			});
		}
	});
}

export function createSampleEntries(handler: (result: { count: number; error?: string }) => void) {
	let succeeded = 0;
	const doNext = (i: number) => {
		if (i == sampleDataEntries.length) {
			handler({ count: succeeded });
		} else {
			createSampleEntry(sampleDataEntries[i], (result) => {
				if (result.error) {
					handler({ count: succeeded, error: result.error });
					return;
				}
				succeeded += result ? 1 : 0;
				doNext(i + 1);
			});
		}
	};
	doNext(0);
}
