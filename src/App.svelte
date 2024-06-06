<script lang="ts">
  import Navbar from './lib/Navbar.svelte';
  import MainView from './lib/MainView.svelte';
</script>


<Navbar />
<main class="flex bg-[#090818] items-center flex-col text-white">
  <div class="text-[42px] text-[#b0afd7] text-center font-['Abril_Fatface'] pt-16 pb-2">Defer!</div>
  <div class="font-['Alef'] text-[14px] w-full text-center text-[#8D8EA6] opacity-70 pb-10 tracking-[0.135em]" id="toptag">need a break?</div>
  <MainView />
</main>
