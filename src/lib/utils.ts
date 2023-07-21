import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function lerp(start: number, end: number, amt: number) {
	return (1 - amt) * start + amt * end;
}

export function map_range(value: number, low1: number, high1: number, low2: number, high2: number) {
	return low2 + ((high2 - low2) * (value - low1)) / (high1 - low1);
}

export function random_in_range(min: number, max: number) {
	return Math.random() * (max - min) + min;
}


export const SHOUT_MAX_LIFETIME = 10000;
export const SHOUT_TICK_CHANGE = 100;
export const SHOUT_PUSH_CHANGE = 2000;
export const UATU_HOST = "https://uatu.fly.dev"; //TODO: FILL!
/*export function uatu(path: string) {
    return "https://corsproxy.io/?" + encodeURIComponent("https://uatu.fly.dev" + path);
}*/