import { writable } from 'svelte/store';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

export const exe_path = writable("");
export const opt_path = writable("");

async function get_config() {
    return await invoke("load_config", {})
}

function update_vals(data: any) {
    exe_path.set(data.binary_path);
    opt_path.set(data.options_path);
}

get_config().then(data => {
    update_vals(data)
})

listen('config_update', (event) => {
    const data = event.payload
    update_vals(data)
})
