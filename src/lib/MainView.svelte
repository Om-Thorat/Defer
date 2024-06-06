<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { sleep } from "../main";

  let autoRestoreIn:number;
  let selectorVisible = true;
  let appString: string;
  let restoreVisible = false;
  let countdownLabel: any;
  let lenApps: number;
  let countdownState = false;
  let screenshotDir: String;

  async function Clean() {
    document.getElementById("toptag").innerText = "ready to get back?";
    console.log(autoRestoreIn);
    selectorVisible = false;

    if (autoRestoreIn == null) {
      countdownLabel = "none";
    } else {
      countdownLabel = `${Math.floor(autoRestoreIn / 60)}m ${autoRestoreIn % 60}s`;
    }

    appString = await invoke("get_apps");
    lenApps = Number(appString[1]);
    appString = appString[0];
    screenshotDir = await invoke("screen_shot");
    restoreVisible = true;

    if (autoRestoreIn != null) {
      countdownLabel = `${Math.floor(autoRestoreIn / 60)}m ${autoRestoreIn % 60}s`;
      countdownState = true;

      for (let i = 1; i <= autoRestoreIn; i++) {
        if (!countdownState) {
          break;
        }
        await sleep(1000);
        countdownLabel = `${Math.floor((autoRestoreIn - i) / 60)}m ${(autoRestoreIn - i) % 60}s`;
      }

      countdownLabel = null;
      restore();
      selectorVisible = true;
    }
  }

  async function restore() {
    document.getElementById("toptag").innerText = "need a break?";
    countdownState = false;
    selectorVisible = true;
    countdownLabel = null;
    restoreVisible = false;
    await invoke("restore");
  }
</script>

{#if restoreVisible}
<div class="gap-5 flex-col items-center hidden transition-all duration-300 opacity-0 -translate-y-[30px] mx-5">
  <div class="w-6 aspect-square rounded-full bg-rose-500 absolute right-[13%] top-[-3%] flex justify-center items-center font-mono">
    {lenApps}
  </div>
  <img class="w-[70%] aspect-auto bg-transparent rounded" src="data:image/png;base64,{screenshotDir}" alt="screenshot" />
  <div class="flex gap-3 flex-col justify-evenly items-center w-[80%]">
    <div class="font-['Alef'] font-bold tracking-[0.04em] max-w-[400px] first-letter:capitalize text-[#8C8CA6] leading-5 max-h-10 overflow-clip text-ellipsis break-words text-center">
      {appString} and more ...
    </div>
    <button on:click={restore} class="transition-all max-w-[400px] duration-300 h-8 w-full bg-[#BEC0E2] text-[#090818] text-[16px] font-sans font-bold text-center tracking-[0.08em] rounded outline-0 hover:outline-[3px] outline outline-emerald-400">
      Restore 🔁
    </button>
  </div>
</div>
{/if}

{#if !restoreVisible}
<button class="w-[75%] max-w-[400px] font-sans font-bold transition-all tracking-[0.08em] duration-300 opacity-100 h-9 bg-[#BEC0E2] text-[#090818] text-[16px] text-center rounded outline-0 hover:outline-[3px] outline outline-emerald-400" on:click={Clean}>
  Defer your apps 👋
</button>
{/if}
<div class="bottom-[3rem] absolute opacity-[0.85] text-sm font-mono">
  Auto restore in
  {#if selectorVisible}
    <select class="bg-[#BEC0E2] text-[#090818] rounded" name="time" bind:value={autoRestoreIn}>
      <option value={null}>none</option>
      <option value="900">15 min</option>
      <option value="2400">40 mins</option>
      <option value="3600">1 hr</option>
    </select>
  {/if}
  <span>{countdownLabel ?? ""}</span>
</div>
