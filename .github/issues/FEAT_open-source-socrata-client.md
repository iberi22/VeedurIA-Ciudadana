---
title: "Publicar Crate Open Source: socrata-rs"
labels:
  - open-source
  - rust
  - library
assignees: []
---

## Contexto
Actualmente Socrata no ofrece un SDK oficial para Rust ([github.com/socrata](https://github.com/socrata)). Como parte del desarrollo de VeedurIA, hemos implementado un cliente robusto (`backend/src/obs/ingest.rs`).

## Objetivo
Refactorizar y extraer nuestra implementación interna para publicarla como el primer crate comunitario de Socrata para Rust.

## Tareas
- [ ] Extraer `src/obs/ingest.rs` a un nuevo workspace/crate `crates/socrata-sdk`.
- [ ] Implementar soporte completo para SoQL (Socrata Query Language) usando el patrón Builder.
- [ ] Agregar documentación (Rustdoc) y ejemplos.
- [ ] Publicar en crates.io como `socrata` o `soda-rs`.
