---
title: "System Architecture"
type: ARCHITECTURE
id: "arch-system"
created: 2026-01-16
updated: 2026-01-16
agent: copilot
model: gemini-3-flash
requested_by: user
summary: |
  Veeduría-IA.co architectural decisions and system design.
keywords: [architecture, design, veeduria, secop, rust, socrata]
tags: ["#architecture", "#design", "#critical", "#veeduria"]
project: veedur-IA.co
---

# 🏗️ Architecture: Veeduría-IA.co

## 🚨 CRITICAL DECISIONS - READ FIRST

> ⚠️ **STOP!** Before implementing ANY feature, verify against this table.
> These decisions are NON-NEGOTIABLE.

| # | Category | Decision | Rationale | ❌ NEVER Use |
|---|----------|----------|-----------|--------------|
| 1 | Hosting | GitHub Pages | Zero cost, Git-native | Vercel, Netlify, AWS |
| 2 | Backend | Rust (Action Runner) | Performance, safety, memory control | Python (Pandas), Node.js |
| 3 | State | GitHub Issues | Protocol compliance & Task tracking | JIRA, Trello |
| 4 | Data Lake | Hugging Face Hub | High volume storage for Parquet | GitHub LFS, AWS S3 |

### Stack
- **Backend:** Rust (ETL, Analysis, IA)
  - `polars`: High-performance data processing
  - `candle`: In-process NLP inference
  - `reqwest`: Socrata API integration
- **Frontend:** Astro 5 + Svelte 5 (Runes)
- **Data:** Parquet (storage), SODA/Socrata (source)
- **Infrastructure:** GitHub Actions (Runner), HF Hub (Persistence)

## Key Decisions

### Decision 1: Rust over Python for ETL
- **Date:** 2026-01-16
- **Context:** GitHub Actions runners have limited RAM (7GB). Loading SECOP II datasets (millions of rows) in Python/Pandas causes OOM.
- **Decision:** Use Rust with Polars (streaming/lazy API) and Candle (quantized BERT) to stay within runner limits.
- **Consequences:** Blazing fast processing, low memory footprint, single binary deployment.

### Decision 2: Hugging Face as "Data Lake"
- **Date:** 2026-01-16
- **Context:** GitHub has strict size limits (~1GB/repo). SECOP II historical data exceeds this.
- **Decision:** Store `.parquet` datasets in Hugging Face Hub. GitHub Actions download/upload only the diffs.
- **Consequences:** Unlimited free storage for open datasets, decoupled from GitHub repo size.

## Project Structure
```
/
├── backend/      # Rust ETL & Analysis
│   ├── src/obs/  # Observation modules (ingest, analyze)
├── frontend/     # Astro + Svelte Portal
├── .gitcore/     # Protocol metadata
└── .github/      # Agents & Workflows
```

## Dependencies (Core)
| Package | Version | Purpose |
|---------|---------|----------|
| polars | 0.45.x | Dataframes |
| candle-core | 0.8.x | ML Inference |
| astro | 5.x | Frontend framework |
| svelte | 5.x | Component framework |
| reqwest | 0.12.x | API Client |

---
*Last updated by GitHub Copilot: 2026-01-16*

