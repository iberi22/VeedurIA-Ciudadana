---
title: "Verificación E2E del Pipeline Completo"
labels:
  - testing
  - quality
assignees: []
---

## Descripción
Validar que el flujo completo desde la ingestión de datos hasta la visualización en la web funcione correctamente en producción.

## Tareas
- [ ] Ejecutar manualmente el workflow `data-pipeline`.
- [ ] Verificar que el commit automático actualice `daily_report.json`.
- [ ] Verificar que GitHub Pages despliegue la nueva versión del sitio.
- [ ] Validar que los contratos recientes aparezcan en el Dashboard.
