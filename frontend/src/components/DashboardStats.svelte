<script lang="ts">
  import { onMount } from 'svelte';

  let stats = $state({
      total_contracts: 0,
      total_value: 0,
      red_flags_count: 0,
      zero_value_count: 0,
      undefined_object_count: 0,
      last_updated: ''
  });

  let loading = $state(true);
  const baseUrl = import.meta.env.BASE_URL || '/';

  onMount(async () => {
    try {
      const res = await fetch(`${baseUrl}stats.json`);
      if (res.ok) {
        stats = await res.json();
      }
    } catch (e) {
      console.error("Failed to load stats", e);
    } finally {
      loading = false;
    }
  });

  function formatMoney(amount: number) {
      return new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', maximumFractionDigits: 0 }).format(amount);
  }
</script>

<div class="relative z-10 mt-auto grid grid-cols-2 gap-4">
    <div class="p-4 rounded-xl bg-black/40 backdrop-blur-sm border border-white/5">
        <div class="text-gray-400 text-xs uppercase">Total Auditado</div>
        <div class="text-white font-bold text-lg">
             {#if loading}
                <span class="animate-pulse">...</span>
             {:else}
                {formatMoney(stats.total_value)}
             {/if}
        </div>
        <div class="text-brand-400 text-sm">
            {#if loading}
                ...
            {:else}
                {stats.total_contracts} Contratos
            {/if}
        </div>
    </div>
    <div class="p-4 rounded-xl bg-black/40 backdrop-blur-sm border border-white/5">
        <div class="text-gray-400 text-xs uppercase">Total Alertas</div>
        <div class="text-white font-bold text-lg">
             {#if loading}
                <span class="animate-pulse">...</span>
             {:else}
                {stats.red_flags_count} Activas
             {/if}
        </div>
        <div class="text-red-400 text-sm">
             {#if loading}
                ...
             {:else}
                {(stats.red_flags_count / (stats.total_contracts || 1) * 100).toFixed(1)}% del total
             {/if}
        </div>
    </div>
</div>
