import { get, writable } from "svelte/store";
import type { IShout } from "./types/IShout";
import { SHOUT_MAX_LIFETIME, SHOUT_PUSH_CHANGE, SHOUT_TICK_CHANGE } from "./utils";

export const shouts_store = writable<IShout[]>([]);

export function add_shout(text: string) {
    shouts_store.update(shouts => {
        shouts.push({
            key: crypto.randomUUID(),
            text,
            lifetime: SHOUT_MAX_LIFETIME
        });
        return shouts;
    });
}

export function push_shout(key: string) {
    shouts_store.update(shouts => {
        let s = shouts.find(e => e.key === key);
        if (s) s.lifetime += SHOUT_PUSH_CHANGE;
        return shouts;
    });
}

export function tick_shouts() {
    let shouts = get(shouts_store);
    for (let shout of shouts) {
        shout.lifetime -= SHOUT_TICK_CHANGE;
        if (shout.lifetime <= 0) {
            shouts = shouts.filter(e => e.key !== shout.key);
        }
    }
    shouts_store.set(shouts);
}