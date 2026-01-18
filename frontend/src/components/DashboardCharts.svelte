<script lang="ts">
  import { onMount } from 'svelte';

  let stats = $state({
      histogram: {
          "0-10M": 0,
          "10M-50M": 0,
          "50M-100M": 0,
          "100M-500M": 0,
          ">500M": 0
      },
      zero_value_count: 0,
      undefined_object_count: 0,
      red_flags_count: 0
  });

  let loading = $state(true);
  const baseUrl = import.meta.env.BASE_URL || '/';

  onMount(async () => {
    try {
      const res = await fetch(`${baseUrl}stats.json`);
      if (res.ok) {
        const data = await res.json();
        // Ensure histogram exists
        if (!data.histogram) {
            data.histogram = { "0-10M": 0, "10M-50M": 0, "50M-100M": 0, "100M-500M": 0, ">500M": 0 };
        }
        stats = data;
      }
    } catch (e) {
      console.error("Failed to load chart stats", e);
    } finally {
      loading = false;
    }
  });

  // Calculate percentages for bars
  function getPercent(value: number, max: number) {
      if (max === 0) return 0;
      return (value / max) * 100;
  }

  let maxBar = $derived(Math.max(...Object.values(stats.histogram)));
</script>

<div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-8">
    <!-- Histogram -->
    <div class="glass-panel p-6 rounded-2xl">
        <h3 class="text-lg font-semibold text-white mb-4">Distribución por Valor (COP)</h3>
        <div class="h-64 flex items-end gap-2 justify-between px-2 pb-4 border-b border-white/5 relative">
            {#each Object.entries(stats.histogram) as [label, count]}
                <div class="w-full bg-brand-500/20 rounded-t hover:bg-brand-500/40 transition-all relative group flex flex-col justify-end"
                     style="height: {getPercent(count, maxBar) || 5}%">
                    <div class="absolute -top-8 left-1/2 -translate-x-1/2 bg-black/80 px-2 py-1 rounded text-xs hidden group-hover:block z-10 whitespace-nowrap border border-white/10">
                        {count} contratos
                    </div>
                </div>
            {/each}
        </div>
        <div class="flex justify-between text-[10px] text-gray-500 mt-2 text-center w-full gap-1">
             {#each Object.keys(stats.histogram) as label}
                <span class="w-full truncate">{label}</span>
             {/each}
        </div>
    </div>

    <!-- Types of Alerts -->
    <div class="glass-panel p-6 rounded-2xl">
        <h3 class="text-lg font-semibold text-white mb-4">Tipos de Alerta</h3>
        <div class="space-y-4">
            <!-- Valor Cero -->
            <div class="space-y-1">
                <div class="flex justify-between text-sm">
                    <span class="text-gray-300">Valor Cero</span>
                    <span class="text-white font-bold">{stats.zero_value_count}</span>
                </div>
                <div class="w-full bg-white/5 rounded-full h-2">
                    <div class="bg-red-500 h-2 rounded-full" style="width: {getPercent(stats.zero_value_count, stats.red_flags_count || 1)}%"></div>
                </div>
            </div>

            <!-- Objeto Indefinido -->
            <div class="space-y-1">
                <div class="flex justify-between text-sm">
                    <span class="text-gray-300">Objeto Indefinido</span>
                    <span class="text-white font-bold">{stats.undefined_object_count}</span>
                </div>
                <div class="w-full bg-white/5 rounded-full h-2">
                    <div class="bg-orange-500 h-2 rounded-full" style="width: {getPercent(stats.undefined_object_count, stats.red_flags_count || 1)}%"></div>
                </div>
            </div>

            <!-- Other placeholder -->
             <div class="space-y-1 opacity-50">
                <div class="flex justify-between text-sm">
                    <span class="text-gray-300">Contratación Directa (Próximamente)</span>
                    <span class="text-white font-bold">-</span>
                </div>
                <div class="w-full bg-white/5 rounded-full h-2">
                    <div class="bg-gray-700 h-2 rounded-full" style="width: 0%"></div>
                </div>
            </div>
        </div>
    </div>
</div>
