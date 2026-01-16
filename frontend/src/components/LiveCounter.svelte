<script lang="ts">
	// Svelte 5 Runes Syntax
	let { count = 0 } = $props();
	let currentCount = $state(count);
	let riskLevel = $derived(currentCount > 50 ? 'High' : currentCount > 20 ? 'Medium' : 'Low');

	function increment() {
		currentCount += 1;
	}
</script>

<div class="glass-panel p-6 rounded-2xl text-center hover:bg-white/10 transition-colors group">
	<div class="text-xs text-gray-400 uppercase tracking-widest mb-2">Contratos Analizados</div>
	<div class="text-5xl font-bold text-white mb-4 transition-all" class:text-red-500={riskLevel === 'High'} class:text-brand-400={riskLevel === 'Low'}>
		{currentCount.toLocaleString()}
	</div>

	<div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/5 border border-white/10 text-xs">
		<span class="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse"></span>
		<span class="text-gray-300">En vivo</span>
	</div>

	<div class="mt-4 opacity-0 group-hover:opacity-100 transition-opacity">
		<button onclick={increment} class="text-xs text-gray-500 hover:text-white underline">Simular Ingesta</button>
	</div>
</div>
