import { db_shouts } from "$lib/server/deta";
import { buildResponse } from "$lib/server/responseHelper";
import type { IShout } from "$lib/types/IShout";
import { SHOUT_MAX_LIFETIME } from "$lib/utils";
import type { RequestHandler } from "./$types";

export const POST = (async ({ request }) => {
	const reqBody = await request.json();

	const { text } = reqBody;

	const shout: IShout = {
		key: crypto.randomUUID(),
		text,
		lifetime: SHOUT_MAX_LIFETIME
	};

    // forward to clients

	return buildResponse().status(200).statusText("OK").json(shout).build();
}) satisfies RequestHandler;
