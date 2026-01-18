---
title: "Implementar Sistema Integral de Master Data Management (MDM)"
labels:
  - enhancement
  - data-engineering
  - rust
  - ai
  - architecture
assignees: []
---

## 🎯 Objetivo General
Integrar una estrategia de **Master Data Management (MDM)** de última generación en `veedur-IA`, capaz de unificar, depurar y certificar los datos provenientes de múltiples fuentes gubernamentales (SECOP II, SIGEP, SIRI), garantizando la creación de "Golden Records" para contratistas y entidades públicas mediante el uso de Inteligencia Artificial y Rust.

## 🏗 Arquitectura de Datos Propuesta (Medallion Architecture)

Adoptaremos una arquitectura por capas ("Medallion") adaptada al patrón **Flat Data** (Serverless/GitHub Actions):

### 1. 🥉 Bronze Layer (Raw Ingestion)
*   **Responsabilidad**: Ingesta fiel de los datos originales sin modificaciones.
*   **Tecnología**: `socrata-sdk` (Rust).
*   **Almacenamiento**: Archivos `.parquet` particionados en Hugging Face Hub (Datasets).
*   **Fuentes**:
    *   SECOP II (Contratos Electrónicos).
    *   SIGEP (Funcionarios).
    *   SIRI (Sanciones).

### 2. 🥈 Silver Layer (Harmonization & MDM Core)
*   **Responsabilidad**: Limpieza, estandarización y **Resolución de Entidades**.
*   **Tecnología**: Motor MDM en Rust (`mdm-core`).
*   **Procesos Clave**:
    *   **Data Quality Gates**: Validación de esquemas y tipos de datos con `Polars`.
    *   **Entity Resolution (IA)**: Identificación de duplicados (e.g., "IBM COLOMBIA" vs "I.B.M. DE COLOMBIA") usando:
        *   *Similitud Léxica*: Jaro-Winkler, Levenshtein.
        *   *Similitud Semántica*: Embeddings con `Candle` (BERT Spanish) + `Qdrant` (Vector DB) para desambiguación compleja.
    *   **Persistent ID Generation**: Asignación de `UUIDs` canónicos a entidades maestras.

### 3. 🥇 Gold Layer (Business & Serving)
*   **Responsabilidad**: Datasets agregados y listos para consumo del Frontend.
*   **Tecnología**: `Polars` -> JSON/Arrow.
*   **Salida**: Archivos estáticos optimizados para la UI (Astro/Svelte).

## 🛠 Tecnologías y Stack

1.  **Rust**: Lenguaje base para todo el pipeline ETL (rendimiento crítico en GitHub Actions).
2.  **Polars**: Motor de procesamiento de dataframes (Lazy evaluation).
3.  **Qdrant**: (Opcional/Híbrido) Base de datos vectorial para búsqueda semántica de entidades y detección de redes de corrupción.
4.  **Candle (Hugging Face)**: Inferencia de ML local (Rust puro) para embeddings de texto.
5.  **Hugging Face Hub**: Almacenamiento "Infinito" para el Data Lake.

## 📋 Roadmap de Implementación

### Fase 1: Fundamentos y Calidad (Bronze -> Silver)
- [ ] **Diseñar Modelos de Dominio**: Definir esquemas canónicos (Structs Rust) para `Contractor`, `PublicEntity`, `Contract`.
- [ ] **Implementar Pipeline de Limpieza**: Crear crate `etl-cleaner` con reglas de normalización (lowercasing, trim, fix encoding).
- [ ] **Data Quality Reports**: Generar reportes automáticos de calidad (nulos, fechas inválidas) en cada ejecución.

### Fase 2: Motor de Resolución de Entidades (The MDM Heart)
- [ ] **Algoritmo de Matching Determinístico**: Implementar reglas exactas (NIT, Cédula).
- [ ] **Algoritmo de Matching Difuso**: Implementar `strsim` para variaciones de nombres.
- [ ] **Integración Vectorial (AI)**:
    - [ ] Implementar generación de embeddings con `Candle` para nombres y objetos contractuales.
    - [ ] Indexar entidades en Qdrant (o índice local HNSW) para búsqueda de vecinos cercanos.

### Fase 3: Gobierno y Stewardship
- [ ] **Sistema de IDs Persistentes**: Mecanismo para mantener IDs estables a través de re-procesamientos.
- [ ] **Dashboard de Linaje**: Visualización simple (Mermaid/Markdown) de de dónde viene cada dato.
- [ ] **Gestión de Excepciones**: Archivo de configuración "manual" (`overrides.toml`) para forzar uniones o separaciones de entidades que la IA no pueda resolver.

## 🔍 Beneficios Esperados
*   **Visión 360° Real**: Ver todos los contratos de una empresa, independientemente de cómo hayan escrito su nombre.
*   **Detección de Redes**: La resolución de entidades es el primer paso para construir el "Grafo de Corrupción".
*   **Datos Confiables**: El Frontend mostrará información certificada, aumentando la credibilidad de la veeduría.
