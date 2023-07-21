<script lang="ts">
	import { shouts_store } from "$lib/shouts_store";
	import { onMount } from "svelte";
import ShoutBox from "../../components/ShoutBox.svelte";
	import TextElement from "../../components/TextElement.svelte";
	import { SHOUT_TICK_CHANGE, UATU_HOST } from "$lib/utils";

    const eventSource = new EventSource(`${UATU_HOST}`, {
            withCredentials: false // TODO: use?
        });
    eventSource.onerror = _onError;
    eventSource.onmessage = _onMessage;

    function _onError(e: any) {
        console.error(e)
    }

    function _onMessage(e: MessageEvent) {
        const data = e.data;
        console.log(data);
    }

    onMount(() => {
        setInterval(() => {
            shouts_store.update(e => {
                e.forEach(shout => {
                    shout.lifetime -= SHOUT_TICK_CHANGE;
                })
                e = e.filter(shout => shout.lifetime > 0);
                return e;
            })
        }, SHOUT_TICK_CHANGE)
    })

</script>

    {#each $shouts_store as shout}
        <TextElement {shout} />
    {/each}

<ShoutBox />