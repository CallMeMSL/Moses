import {type Writable, writable} from 'svelte/store';
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";

export const is_in_debug = writable(null);

async function is_debug() {
    return await invoke("is_in_debug", {})
}

function set_is_debug(data: any) {
    is_in_debug.set(data)
}



is_debug().then(data => {
    set_is_debug(data)
})

listen('debug_file_update', (event) => {
    const data = event.payload
    set_is_debug(data)
})
