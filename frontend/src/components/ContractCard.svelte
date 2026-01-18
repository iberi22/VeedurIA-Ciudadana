<script lang="ts">
  let { contrato } = $props();

  // Get base URL for GitHub Pages compatibility
  const baseUrl = import.meta.env.BASE_URL || '/';

  function navigateToReport() {
    const params = new URLSearchParams({
      id: contrato.id_contrato,
      entity: contrato.nombre_entidad,
      type: contrato.tipo_de_contrato
    });
    window.location.href = `${baseUrl}denunciar?${params.toString()}`;
  }

  const valor = contrato.valor_del_contrato || "$0";
  const entidad = contrato.nombre_entidad || "Entidad Desconocida";
  const objeto = contrato.objeto_del_contrato || "Sin descripción";
  const contratista = contrato.nombre_contratista || "Por definir";
  // Mock risk logic for now based on some properties or random to demonstrate
  const isHighRisk = valor.length > 10; // Placeholder logic
</script>

<div
  onclick={navigateToReport}
  class="p-4 rounded-xl bg-white/5 border border-white/5 hover:border-brand-500/30 transition-all cursor-pointer group active:scale-[0.98]"
>
  <div class="flex justify-between items-start mb-2">
    {#if isHighRisk}
      <span class="px-2 py-1 rounded text-xs font-bold bg-red-500/20 text-red-400 border border-red-500/20">ALTO RIESGO</span>
    {:else}
      <span class="px-2 py-1 rounded text-xs font-bold bg-brand-500/20 text-brand-400 border border-brand-500/20">NORMAL</span>
    {/if}
    <span class="text-gray-500 text-xs">{contrato.fecha_de_firma || 'Reciente'}</span>
  </div>

  <h4 class="text-white font-medium line-clamp-2 group-hover:text-brand-400 transition-colors">
    {objeto}
  </h4>
  <p class="text-gray-400 text-sm mt-2">{entidad}</p>

  <div class="mt-3 flex items-center justify-between text-xs text-gray-500 border-t border-white/5 pt-3">
    <span class="font-mono text-brand-300">{valor}</span>
    <span class="opacity-70">{contratista.split(' ')[0]}...</span>
  </div>
</div>
