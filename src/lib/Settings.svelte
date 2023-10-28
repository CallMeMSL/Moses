<script lang="ts">
    import {RangeSlider} from "@skeletonlabs/skeleton";
    import {invoke} from "@tauri-apps/api/tauri";
    import {exe_path, opt_path} from "../stores/config-store";

    async function choose_exe() {
        await invoke("choose_binary")
    }

    async function choose_options() {
        await invoke("choose_options")
    }

    let ep: String;
    exe_path.subscribe((v) => {
        ep = v
    })

    let op: String;
    opt_path.subscribe((v) => {
        op = v
    })

    let startup_timer_val = 0;


</script>
<div class="relative top-1/3">
    <div>
        <form on:submit|preventDefault={choose_exe}>
            <p class="italic">Executable:</p>
            <div class="flex">
                <button type="submit" class="btn variant-filled rounded-r-2xl p-1">
                    Select
                </button>

                <input disabled class="input rounded-l" bind:value={ep}>

            </div>
        </form>
    </div>
    <div>
        <form on:submit|preventDefault={choose_options}>
            <p class="italic">Options.ini:</p>
            <div class="flex">
                <button type="submit" class="btn variant-filled rounded-r-2xl p-1">
                    Select
                </button>
                <input disabled class="input rounded-l" bind:value={op}>
            </div>
        </form>
    </div>
</div>
<!--<br>-->
<!--<RangeSlider name="range-slider" bind:value={startup_timer_val} max={60} step={1} ticked>-->
<!--    Auto start app in {startup_timer_val}s (disabled at 0):-->
<!--</RangeSlider>-->
