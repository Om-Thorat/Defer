<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { sleep } from '../main';
    let AutoIn:any = "none";
    let selector:HTMLSelectElement;
    let greetMsg:string = "No Windows th u mean there are lots lots!";
    let RestoreDiv:HTMLElement;
    let FoldDiv:HTMLElement;
    let countdown:any;
    let lenapps:number;
    let CDownState:number = 0;
    let SSdir:String;
    async function Clean(){
      document.getElementById("toptag").innerText = "ready to get back?"
      console.log(AutoIn);
      selector.classList.add("hidden");
      if (AutoIn == "none"){
        countdown = AutoIn;
      }
      else{
        countdown = `${Math.floor(AutoIn/60)}m ${AutoIn%60}s`;
      }
      FoldDiv.style.opacity = "0.2";
      greetMsg = await invoke('get_apps');
      lenapps = greetMsg[1];
      greetMsg = greetMsg[0];
      SSdir = await invoke('screen_shot');
      await sleep(20)
      FoldDiv.style.opacity = "0%";
      FoldDiv.style.transform = "translate(0px,30px)";
      await sleep(180)
      FoldDiv.classList.add("hidden");
      RestoreDiv.style.opacity = "1";
      RestoreDiv.style.transform = "translate(0px,0px)"
      RestoreDiv.classList.replace("hidden","flex");
      RestoreDiv.style.opacity = "1";
      if (AutoIn != "none"){
        countdown = `${Math.floor(AutoIn/60)}m ${AutoIn%60}s`;
        CDownState = 1;
        for (let i = 1; i <= AutoIn; i++) {
          if (CDownState == 0){
            break
          }
          await sleep(1000)
          countdown = `${Math.floor((AutoIn - i)/60)}m ${(AutoIn - i)%60}s`        
        }
        countdown = undefined;
        restore();
        selector.classList.remove("hidden");
      }
    }
    async function restore() {
      document.getElementById("toptag").innerText = "need a break?"
      CDownState = 0
      selector.classList.remove("hidden");
      countdown = undefined;
      RestoreDiv.style.opacity = "0%";
      RestoreDiv.style.transform = "";
      await sleep(180)
      RestoreDiv.classList.replace("flex","hidden");
      FoldDiv.style.transform = "translate(0px,0px)";
      FoldDiv.style.opacity = "1";
      FoldDiv.classList.remove("hidden");
      await invoke("restore");
    }
</script>

<div class="gap-5 flex-col items-center hidden transition-all duration-300 opacity-0 -translate-y-[30px] mx-5" bind:this={RestoreDiv}>
    <div class=" w-6 aspect-square rounded-full bg-rose-500 absolute right-[13%] top-[-3%] flex justify-center items-center font-mono">{lenapps}</div>
    <img class="w-[70%] aspect-auto bg-transparent rounded" src="data:image/png;base64,{SSdir}" alt="screenshot">
    <div class="flex gap-3 flex-col justify-evenly items-center w-[80%]">
    <div class="font-['Alef'] font-bold tracking-[0.04em] max-w-[400px] first-letter:capitalize text-[#8C8CA6] leading-5 max-h-10 overflow-clip text-ellipsis break-words text-center">
      {greetMsg} and more ...
    </div>
    <button on:click={restore} class="transition-all max-w-[400px] duration-300 h-8 w-full bg-[#BEC0E2] text-[#090818] text-[16px] font-sans font-bold text-center tracking-[0.08em] rounded outline-0 hover:outline-[3px] outline outline-emerald-400">Restore 🔁</button>
    </div>
</div>

<button bind:this={FoldDiv} class="w-[75%] max-w-[400px] font-sans font-bold transition-all tracking-[0.08em] duration-300 opacity-100 h-9 bg-[#BEC0E2] text-[#090818] text-[16px] text-center rounded outline-0 hover:outline-[3px] outline outline-emerald-400" on:click={Clean}>
  Defer your apps 👋
</button>


<div class="bottom-[3rem] absolute opacity-[0.85] text-sm font-mono ">Auto restore in <select bind:this={selector} class="bg-[#BEC0E2] text-[#090818] rounded" name="time" bind:value={AutoIn}>
  <option value="none">none</option>
  <option value="900">15 min</option>
  <option value="2400">40 mins</option>
  <option value="3600">1 hr</option>
</select> 
<span>{countdown ?? ""}</span>
</div>