<script lang="ts">
	import { Button } from "$components/ui/button";
	import { Input } from "$components/ui/input";
	import { shouts_store } from "$lib/shouts_store";
	import { SHOUT_MAX_LIFETIME, UATU_HOST } from "$lib/utils";

    let inpValue: string = "";

    async function onShout() {

        shouts_store.update(e => {
            e.push({
                key: crypto.randomUUID(),
                text: inpValue,
                lifetime: SHOUT_MAX_LIFETIME
            });
            return e;
        })
        inpValue = "";
        const res = await fetch(`${UATU_HOST}/push/default`, {
            method: "POST",
            body: JSON.stringify({
                action: "new",
                data: {
                    text: inpValue
                }
            })
        })

        inpValue = "";
    }
</script>

<div id="shoutbox" class="fixed w-full bottom-0 pb-6 px-6 flex flex-col justify-center items-center">
	<form class="flex w-full max-w-md items-center space-x-2" on:submit|preventDefault={onShout}>
		<Input type="text" placeholder="What's on your mind? 💬" bind:value={inpValue}/>
		<Button type="submit">Shout</Button>
	</form>
    <br>
    <p class="text-sm text-muted-foreground">Let other's join this session at: <a href="/" class="text-blue-700">https://localhost:4211/app</a></p>
</div>

<style lang="postcss">
</style>
