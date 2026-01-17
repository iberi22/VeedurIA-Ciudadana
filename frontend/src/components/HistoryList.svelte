<script lang="ts">
  import { onMount } from 'svelte';

  let history = $state([]);

  onMount(() => {
    loadHistory();
    // Listen to storage changes (if improved later) or manual refresh could be added
  });

  function loadHistory() {
    try {
      history = JSON.parse(localStorage.getItem('veeduria_reports') || '[]');
    } catch {
      history = [];
    }
  }

  function clearHistory() {
    localStorage.removeItem('veeduria_reports');
    history = [];
  }
</script>

{#if history.length > 0}
  <div class="glass-panel p-6 rounded-2xl md:col-span-2 mt-8">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-xl font-bold text-white">Mis Denuncias Recientes</h3>
      <button onclick={clearHistory} class="text-xs text-red-400 hover:text-red-300">
        Borrar Historial
      </button>
    </div>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each history as item}
        <div class="p-3 bg-white/5 rounded-lg border border-white/5 flex flex-col gap-1">
          <div class="flex justify-between">
            <span class="font-mono text-brand-300 text-sm">{item.id}</span>
            <span class="text-xs text-gray-400">{new Date(item.date).toLocaleDateString()}</span>
          </div>
          <p class="text-white text-sm line-clamp-1">{item.entity}</p>
          <span class="text-xs text-gray-500 bg-white/5 px-2 py-0.5 rounded w-fit">
            {item.type}
          </span>
        </div>
      {/each}
    </div>
  </div>
{/if}
