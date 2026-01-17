import { writable } from 'svelte/store';

export const reportData = writable({
    contractId: '',
    irregularityType: 'Seleccione una opción...',
    description: '',
    entity: 'Entidad de Ejemplo',
    amount: '$0',
});
