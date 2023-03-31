<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
</script>

<div>
  <div style="display: flex; flex-direction: column; gap: 2em; align-items: center;">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet} style="width: 50%;">
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>