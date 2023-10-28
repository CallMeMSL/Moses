<script lang="ts">
    import Trophy from '../ico/Trophy.svelte';
    import {invoke} from "@tauri-apps/api/tauri";
    import {getToastStore, type ToastSettings} from '@skeletonlabs/skeleton';
    import {is_in_debug} from "../stores/debug-store";

    const toastStore = getToastStore();

    async function toggle_debug() {
        const res: string = await invoke("toggle_debug")
        if (res !== "") {
            const t: ToastSettings = {
                message: 'An error was encountered during mode switch.\n' + res,
                background: 'variant-filled-error',
            };
            toastStore.trigger(t);
        }
    }
    let launchtext;
    is_in_debug.subscribe((v) => {
        if (v) {
            launchtext = "start with debug console enabled"
        } if (v == null) {
            launchtext = "set a valid binary and options file"
        } if (v == false) {
            console.log(v)
            launchtext = "start in daily run mode"
        }
    })
    $: disabled = launchtext === "set a valid binary and options file";

    async function start_isaac() {
        await invoke("start_isaac")
    }

</script>

<div class="flex justify-evenly">
    <div>
        <button disabled={disabled || null} on:click={start_isaac} type="button" class="w-80 btn-xl bg-blue-600 text-center rounded h-60">
<!--            <Trophy></Trophy>-->
            {launchtext}
        </button>
    </div>
    <div class="flex items-center justify-center">
        <button type="button" class=" btn variant-filled rounded" on:click={toggle_debug}>Swap Launch Mode</button>
    </div>
</div>