<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'
    async function close() {
        appWindow.hide()
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div data-tauri-drag-region class="flex gap-2 justify-end text-white pt-1 pr-2font-mono font-semibold ">
    <div class="transition-all duration-200 justify-center items-center flex font-['Alef'] tracking-[0.135em] text-[12px] cursor-pointer opacity-60 hover:opacity-100" on:click={() => appWindow.minimize()}>
        min
    </div>
    <div class="transition-all duration-200 justify-center items-center flex font-['Alef'] tracking-[0.135em] text-[12px] cursor-pointer opacity-60 hover:opacity-100" on:click={() => appWindow.toggleMaximize()}>
        max
    </div>
    <div class="transition-all duration-200 justify-center items-center flex font-['Alef'] tracking-[0.135em] text-[12px] cursor-pointer opacity-60 hover:opacity-100 pr-2" on:click={() => close()}>
        quit
    </div>
</div>
