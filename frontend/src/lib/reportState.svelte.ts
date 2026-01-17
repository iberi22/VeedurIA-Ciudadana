export class ReportState {
  contractId = $state('');
  irregularityType = $state('Seleccione una opción...');
  description = $state('');
  entity = $state('Entidad de Ejemplo');
  amount = $state('$0');

  // Validation: At least 6 alphanumeric chars, allowing dots and dashes
  isValidContractId = $derived(/^[a-zA-Z0-9.-]{6,}$/.test(this.contractId));

  reset() {
    this.contractId = '';
    this.irregularityType = 'Seleccione una opción...';
    this.description = '';
  }
}

export const reportState = new ReportState();
