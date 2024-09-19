import type { Task } from "$lib/types";

export async function load({ fetch, params }) {
  let C2_URL = import.meta.env.VITE_C2_URL;
  const response = await fetch(`${C2_URL}/tasks/${params.slug}`);
  const task: Task = await response.json();
  return {
    props: {
      task,
    },
  };
}