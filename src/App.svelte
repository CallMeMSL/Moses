<script lang="ts">
    import Launch from "./lib/Launch.svelte";
    import Settings from "./lib/Settings.svelte";
    import {initializeStores, Toast} from '@skeletonlabs/skeleton';

    initializeStores();
</script>

<main class="container mx-auto">
    <Toast />

    <div class="flex flex-col h-screen p-4">
        <div class="h-1/2">
            <Launch></Launch>
        </div>
        <div class="h-1/2 ">
            <Settings></Settings>
        </div>
    </div>

</main>

<style>

</style>