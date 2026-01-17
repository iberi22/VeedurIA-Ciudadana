<script lang="ts">
  import { onMount } from 'svelte';
  import { reportState } from '../lib/reportState.svelte';

  onMount(() => {
    const params = new URLSearchParams(window.location.search);
    if (params.has('id')) {
      reportState.contractId = params.get('id') || '';
      reportState.entity = params.get('entity') || '';
      // Mapping simple types or default
      const type = params.get('type') || '';
      if (['Sobreprecios', 'Direccionamiento', 'Falsedad', 'Incumplimiento'].includes(type)) {
        reportState.irregularityType = type;
      }
    }
  });
</script>

<div class="glass-panel p-8 rounded-2xl">
  <h2 class="text-2xl font-bold text-white mb-6 flex items-center gap-3">
    <span class="flex items-center justify-center w-8 h-8 rounded-full bg-brand-500 text-white text-sm">1</span>
    Detalles del Contrato
  </h2>

  <form class="space-y-6">
    <div>
      <label class="block text-sm font-medium text-gray-400 mb-2">ID del Contrato (SECOP)</label>
      <div class="relative">
        <input
          type="text"
          bind:value={reportState.contractId}
          placeholder="Ej: CO1.PCCNTR.123456"
          class="w-full bg-obsidian-900/50 border rounded-lg px-4 py-3 text-white focus:outline-none transition-colors {
            reportState.contractId && !reportState.isValidContractId ? 'border-red-500 focus:border-red-500' :
            reportState.contractId && reportState.isValidContractId ? 'border-green-500 focus:border-green-500' :
            'border-white/10 focus:border-brand-500'
          }"
        />
        {#if reportState.contractId}
          <div class="absolute right-3 top-3 text-xs">
            {#if reportState.isValidContractId}
              <span class="text-green-500 font-bold">✓ Válido</span>
            {:else}
              <span class="text-red-500 font-bold">✕ Inválido</span>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-400 mb-2">Tipo de Irregularidad</label>
      <select
        bind:value={reportState.irregularityType}
        class="w-full bg-obsidian-900/50 border border-white/10 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-brand-500 transition-colors"
      >
        <option>Seleccione una opción...</option>
        <option>Sobreprecios</option>
        <option>Direccionamiento (Pliegos Sastre)</option>
        <option>Falsedad en Documentos</option>
        <option>Incumplimiento de Ejecución</option>
      </select>
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-400 mb-2">Descripción Adicional</label>
      <textarea
        rows="4"
        bind:value={reportState.description}
        placeholder="Describa brevemente los hechos..."
        class="w-full bg-obsidian-900/50 border border-white/10 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-brand-500 transition-colors"
      ></textarea>
    </div>
  </form>
</div>
