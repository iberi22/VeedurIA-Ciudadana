---
title: "Implementar Motor de Generación de Denuncias"
labels:
  - frontend
  - feature
assignees: []
---

## Descripción
Crear la lógica para transformar los datos de un contrato "Red Flag" en un formato de denuncia formal para la Procuraduría.

## Tareas
- [ ] Crear plantillas de texto para diferentes tipos de irregularidades (Sobrecosto, Plazos cortos, Único proponente).
- [ ] Implementar generador de PDF (usando `pdf-lib` o similar en JS client-side) o modal de "Copiar al portapapeles".
- [ ] Botón de "Generar Evidencia" en `RedFlagCard.svelte`.
