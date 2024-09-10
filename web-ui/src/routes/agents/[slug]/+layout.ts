import {error} from '@sveltejs/kit';
import type { Agent } from '$lib/types';

export async function load({fetch, params}) {
    let C2_URL = import.meta.env.VITE_C2_URL;
    try {
        const response = await fetch(`${C2_URL}/agents/${params.slug}`);
        const agent: Agent = await response.json();
        return {
            agent,
            slug: params.slug
        };
    } catch (er) {
        throw error(404, `Could not get agent`)
    }
}