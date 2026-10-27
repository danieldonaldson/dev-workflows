# Extraction backlog

Candidates found while scanning `~/Work` (2026-10-27), roughly by leverage.

## High

- [ ] **Rust/Axum backend template.** An auth slice was already ported into toutouwai by hand.
      A template repo with hexagonal layout, auth (Argon2id, RS256, rotating refresh tokens,
      CSRF, OAuth), `Settings::from_env` with aggregated errors, `persist_err` mapping,
      leader election, `sqlx::query!` + `.sqlx`, nextest, clippy pedantic, cargo deny/audit,
      scratch Dockerfile. Sources: `toutouwai/backend`, plus a SQLite variant.
- [ ] **Static site on Cloudflare Workers runbook.**
      `wrangler.jsonc` static assets, `public/_headers`, Custom Domains vs
      Routes (API error 100117 if a record exists), `pnpm run deploy` (bare `pnpm deploy` is a
      built-in), observability limits of the OAuth token.
- [ ] **Rust → TypeScript Worker rewrite**: what changed, Turnstile/
      reCAPTCHA siteverify, Resend sending domains. Some of this is in Claude memory already.

## Medium

- [ ] **justfile skeleton** for a new project: sections (daily loop / rare provisioning / smoke
      tests), `preflight`, `ci` mirror. Source: toutouwai.
- [ ] **Tiered container images via BuildKit bake + named contexts** (toutouwai,
      `test-local.sh`: build, flatten, smoke-test with prod hardening, no push).
- [ ] **Network-policy templating** (`gen-policies.sh` with `--check`) — generic per-env renderer.
- [ ] **Talos-in-QEMU local cluster** (`k8s/local/*.sh`, `preflight.sh`, Tiltfile, in-cluster git
      daemon) as a standalone starter.
- [ ] **Eval/spike write-up format** — a `FINDINGS-*.md` series is a good shape for
      time-boxed technology evaluations.
- [ ] **Public framework + private overlay** repo pattern.

## Low / check first

- [ ] Statusline tool — already standalone; just link it.
- [ ] Two projects that look like upstream forks — confirm before extracting.
- [ ] VPS/Dokploy setup — superseded by the Workers move; archive notes only.
