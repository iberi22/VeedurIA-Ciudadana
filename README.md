# Veeduría Ciudadana: Portal Nacional de Control Fiscal

> **Democratizando el control fiscal con Datos Abiertos e Inteligencia Artificial.**

Este proyecto tiene como objetivo desarrollar una plataforma pública, robusta y estéticamente impactante para permitir a cualquier ciudadano colombiano realizar veeduría sobre la contratación pública (SECOP II). El sistema utiliza técnicas avanzadas de detección de anomalías (Ley de Benford, NLP con Modelos de Lenguaje) para identificar "Red Flags" de corrupción y facilitar la denuncia ante la Procuraduría.

## 🚀 Misión
Transformar la veeduría de una tarea manual y reactiva a una **sistemática, preventiva y asistida por IA**, empoderando a la ciudadanía con herramientas de nivel profesional.

---

## 🛠️ Stack Tecnológico

### Frontend (Portal Público)
-   **Framework**: [Astro 5](https://astro.build) (SSG para rendimiento y SEO)
-   **Interactividad**: [Svelte 5](https://svelte.dev) (Runes API para reactividad)
-   **Estilos**: TailwindCSS v4 (Diseño "Glassmorphism" y Dark Mode)
-   **Gráficos**: D3.js / Chart.js (Visualización de datos)

### Backend (Análisis & Datos)
-   **Lenguaje**: [Rust](https://www.rust-lang.org) (Rendimiento crítico y seguridad de memoria)
-   **ETL & Dataframes**: [Polars 0.45+](https://pola.rs) (Lazy API para millones de registros)
-   **IA/ML**: [Candle 0.8+](https://github.com/huggingface/candle) (Inferencia de modelos BERT en Rust)
-   **Infraestructura**: GitHub Actions (Orquestación) + Hugging Face Hub (Datasets Parquet)

---

## 📡 Integración con APIs Gubernamentales (SODA/Socrata)

### Registro de App Token en datos.gov.co
Para acceder a la API SODA con límites de tasa elevados (1,000 req/hora), se requiere registrar la aplicación:

1.  **Crear cuenta** en [datos.gov.co](https://www.datos.gov.co)
2.  **Ir a "Mi Perfil"** → Sección "Aplicaciones" → "Administrar"
3.  **Crear nueva aplicación**: Proporcionar nombre único y descripción
4.  **Obtener App Token**: Socrata genera un token alfanumérico único
5.  **Configurar en el proyecto**: Guardar en `.env` como `SOCRATA_APP_TOKEN`

### Datasets Clave
| Dataset | ID Socrata | Uso |
|---------|------------|-----|
| SECOP II Contratos | `jbjy-vk9h` | Análisis de contratos |
| SECOP II Procesos | `p6dx-8zbt` | Tiempos de licitación |
| SIGEP Funcionarios | `2jzx-383z` | Cruce de conflictos |

---

## 🤖 Integración con Hugging Face Hub

### Estrategia de Almacenamiento de Datasets
GitHub tiene límites de 100MB por archivo y ~1GB por repositorio. Para datasets masivos (SECOP II tiene millones de registros), usamos **Hugging Face Hub** como "Data Lake".

### Flujo de Datos
```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Socrata API │────▶│  Rust Binary │────▶│  Parquet     │────▶│  HF Hub      │
│  (datos.gov) │     │  (Polars)    │     │  (Columnar)  │     │  Dataset     │
└──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
                                                                      │
                                                                      ▼
                                          ┌──────────────┐     ┌──────────────┐
                                          │  Astro SSG   │◀────│  GitHub      │
                                          │  (Frontend)  │     │  Pages       │
                                          └──────────────┘     └──────────────┘
```

### Comandos de Sincronización
```bash
# Descargar dataset desde Hugging Face
huggingface-cli download iberi22/veeduria-secop-ii --repo-type dataset

# Subir dataset actualizado (después del procesamiento diario)
huggingface-cli upload iberi22/veeduria-secop-ii ./data --repo-type dataset
```

### Configuración del Repositorio HF
1.  Crear cuenta en [huggingface.co](https://huggingface.co)
2.  Crear nuevo Dataset: `iberi22/veeduria-secop-ii`
3.  Generar token de escritura en Settings → Access Tokens
4.  Guardar en `.env` como `HF_TOKEN`

---

## 🗺️ Roadmap & Plan de Implementación

### Fase 1: Cimientos y Diseño ✅
- [x] Análisis de requisitos (`Veeduría-main-doc.md`)
- [x] Planificación de Arquitectura Técnica
- [x] Diseño de UI/UX (Mockups de Alta Fidelidad)
- [x] Inicialización del proyecto Frontend (Astro + Svelte 5)
- [x] Configuración del Sistema de Diseño (Glassmorphism)

### Fase 2: Desarrollo del Frontend ✅
- [x] **Landing Page**: Contador en tiempo real, Hero section
- [x] **Dashboard de "Red Flags"**: Visualización diaria/semanal/mensual
- [x] **Centro de Denuncias**: Generador de oficios

### Fase 3: Ingeniería de Datos (Backend Rust) 🔄
- [x] Inicialización del proyecto Rust
- [ ] Registro de App Token en datos.gov.co (Socrata)
- [ ] Cliente Socrata API con `reqwest`
- [ ] Algoritmo de Ley de Benford con `polars`
- [ ] Pipeline CI/CD en GitHub Actions
- [ ] Integración con Hugging Face Hub

### Fase 4: Inteligencia Artificial
- [ ] Integración de modelos NLP (BERT español) con Candle
- [ ] Clasificación automática de objetos contractuales
- [ ] Detección de similitud semántica (contratos fraccionados)

---

## 📂 Estructura del Repositorio

```
veedur-IA.co/
├── README.md               # Este archivo
├── .gitignore              # Archivos ignorados
├── Veeduría-main-doc.md    # Documento técnico de referencia
├── frontend/               # Aplicación Astro + Svelte
│   └── src/
│       ├── pages/          # Rutas (index, dashboard, denunciar)
│       ├── components/     # Componentes Svelte (LiveCounter)
│       ├── layouts/        # Layout principal
│       └── styles/         # Tailwind v4 global.css
└── backend/                # Binarios Rust para ETL y Análisis
    └── src/
        ├── main.rs         # Punto de entrada
        └── obs/            # Módulo de observación (ingest, analyze)
```

---

## 🤝 Cómo Contribuir
Este es un proyecto de código abierto. Las instrucciones de instalación y contribución se detallarán próximamente.

### Desarrollo Local
```bash
# Frontend
cd frontend && npm install && npm run dev

# Backend
cd backend && cargo run
```

---

**Desarrollado para Colombia 🇨🇴**
