<script lang="ts">
    import {writable} from 'svelte/store';
    import {type HistoryEntry} from '$lib/commands';
    import {Heading} from 'flowbite-svelte';
    import HistoryPanel from '$lib/components/HistoryPanel.svelte';
    import InfoPanel from '$lib/components/InfoPanel.svelte';
    import TopBar from '$lib/components/TopBar.svelte';

    let history = writable<HistoryEntry[]>([]);
    let selected = $state<HistoryEntry | undefined>(undefined);
</script>

<TopBar {history} bind:selected />
<div class="grid grid-cols-2 gap-4 p-4">
    <div class="col-span-1 space-y-4">
        <div class="flex justify-center">
            <Heading tag="h5">Entry History</Heading>
        </div>
        <div class="flex justify-center">
            <HistoryPanel {history} bind:selected />
        </div>
    </div>
    <div class="col-span-1 space-y-4">
        <InfoPanel {selected} />
    </div>
</div>
