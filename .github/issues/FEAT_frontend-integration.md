---
title: "Integrar daily_report.json en Dashboard Frontend"
labels:
  - frontend
  - integration
assignees: []
---

## Descripción
Conectar los componentes Svelte del Dashboard con el archivo `daily_report.json` generado por el backend.

## Tareas
- [ ] Definir interface TypeScript para `ContratoSecop` en el frontend.
- [ ] Actualizar `Dashboard.svelte` para cargar `daily_report.json` al inicio.
- [ ] Mapear los datos de "Benford" y "NLP" a los gráficos de visualización.
- [ ] Implementar manejo de estados de carga y error si el JSON no existe.
