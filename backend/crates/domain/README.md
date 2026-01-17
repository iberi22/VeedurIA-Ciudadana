# Veeduría Domain

Core business models and rules for the Veeduría-IA.co platform.

## Purpose
This crate contains the **Pure Domain** logic, following Hexagonal Architecture principles. It has zero external dependencies (other than Serde for serialization) and defines the core entities used by both the backend and future modules.

## Entities
- `ContratoSecop`: The standard model for a Colombian public contract.

## Principles
- No IO (Input/Output).
- No external infrastructure dependencies.
- High cohesion, low coupling.
