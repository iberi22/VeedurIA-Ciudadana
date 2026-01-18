<script lang="ts">
  import { onMount } from 'svelte';
  import ContractCard from './ContractCard.svelte';

  let contracts = $state([]);
  let loading = $state(true);
  let error = $state(null);

  // Get base URL for GitHub Pages compatibility
  const baseUrl = import.meta.env.BASE_URL || '/';

  onMount(async () => {
    try {
      const res = await fetch(`${baseUrl}daily_report.json`);
      if (!res.ok) throw new Error('No se pudo cargar el reporte diario.');
      contracts = await res.json();
    } catch (e) {
      error = e.message;
      // Fallback to empty if not found
      contracts = [];
    } finally {
      loading = false;
    }
  });
</script>

<div class="glass-panel rounded-2xl p-6 h-[500px] flex flex-col">
  <h3 class="text-xl font-semibold mb-6 text-white sticky top-0 bg-obsidian-950/80 backdrop-blur-md py-2 -mx-2 px-2 z-10 flex justify-between items-center">
    Reporte Diario
    {#if !loading}
      <span class="text-xs font-normal text-gray-500">{contracts.length} hallazgos</span>
    {/if}
  </h3>

  <div class="space-y-4 overflow-y-auto pr-2 custom-scrollbar flex-1">
    {#if loading}
      {#each Array(3) as _}
        <div class="h-32 rounded-xl bg-white/5 animate-pulse border border-white/5"></div>
      {/each}
    {:else if error}
      <div class="text-center py-10">
        <p class="text-red-400 text-sm">{error}</p>
        <p class="text-gray-500 text-xs mt-2">Asegúrate de que el backend haya generado el reporte.</p>
      </div>
    {:else if contracts.length === 0}
      <div class="text-center py-10">
        <p class="text-gray-400 text-sm">No se encontraron irregularidades hoy.</p>
      </div>
    {:else}
      {#each contracts as contrato}
        <ContractCard {contrato} />
      {/each}
    {/if}
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
</style>
