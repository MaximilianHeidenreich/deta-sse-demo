<script lang="ts">
	import type { IShout } from "$lib/types/IShout";
	import { UATU_HOST, lerp, map_range, random_in_range } from "$lib/utils";
	import { onMount } from "svelte";


    export let shout: IShout;
    let scale = 100;

    let padding = 200;
    let posX = random_in_range(padding, window.innerWidth - padding);
    let posY = random_in_range(padding, window.innerHeight - padding);

    $: scale = map_range(shout.lifetime, 0, 10000, 0, 100);

    onMount(() => {
    })

    async function onClick() {
        const res = await fetch(`${UATU_HOST}/push/default`, {
            method: "POST",
            body: JSON.stringify({
                action: "push",
                data: {
                    key: shout.key
                }
            })
        })
    }

</script>

<h1 style="--scale: {scale}%; --posX: {posX}; --posY: {posY}" class="text-6xl font-medium cursor-pointer" on:click={onClick}>
    {shout.text}
</h1>

<style lang="postcss">
    h1 {
        /*position: absolute;
        left: var(--posX);
        top: var(--posY);
        transform: translateX(var(--posX)) translateY(var(--posY)) scale(var(--scale));*/
        transform: scale(var(--scale));
    }
</style>