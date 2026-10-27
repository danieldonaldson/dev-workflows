# dev-workflows

Copy-pasteable **how**: scripts, task-runner modules, manifests, runbooks and templates
extracted from real projects. The **why** lives in the notes vault (`~/notes/atomic/Dev/`),
where each lesson is an atomic note; files here link back to them.

Rule of thumb: if it's a decision or a lesson → notes. If you'd copy it into a new repo → here.

**Starting something new?** Begin with the checklist in the vault: `~/notes/atomic/Dev/Starting a new project.md`.
It walks through day-one decisions in order and points at the files here.

## Contents

| Path | What | Origin |
|------|------|--------|
| `just/common.just` | pnpm supply-chain wrapper, clean-tree gate, confirm prompt, `sops exec-env` runner, recipe hygiene checklist | toutouwai |
| `just/kube-context.just` | `just ctx local\|prod` — per-shell, fail-closed kube/talos context + `_require-ctx` guard | toutouwai |
| `ci/pre-commit.sh` | Path-scoped pre-commit hook (Rust fmt/clippy, eslint, generated-file `--check`) | toutouwai |
| `k8s/install-cilium-talos-gvisor.sh` | Cilium Helm install for Talos (KubePrism, kube-proxy replacement, gVisor socket-LB fix) | toutouwai |
| `k8s/vap-tenant-guardrails.yaml` | ValidatingAdmissionPolicy: require gVisor, no SA token, approved image prefixes, policy label — fail-closed CEL | toutouwai |
| `k8s/reseal.sh` + `k8s/sops.yaml.example` | SOPS source → SealedSecret, nothing plaintext on disk | toutouwai |
| `k8s/snippets.yaml` | ArgoCD CRD-instance waves, CronJob TTL, Recreate for hostPort, CNPG `bootstrap: recovery` | toutouwai |
| `web/nginx.conf` + `web/nginx-headers.spec.ts` | Static-site nginx with header inheritance made structurally safe, and the Playwright guard test | toutouwai |
| `web/dockerignore.allowlist` | `.dockerignore` as an allowlist for monorepo-root frontend builds | toutouwai |
| `rust/leader.rs` | Postgres advisory-lock leader election with heartbeat + graceful shutdown | toutouwai |
| `rust/env.rs` | Empty-is-unset env helpers, aggregated required-var check | toutouwai |
| `runbooks/talos-cluster-bringup.md` | Ordered cloud Talos → Cilium → ArgoCD bring-up with the gotchas inline | toutouwai |
| `runbooks/dr-rehearsal.md` | Wipe-and-restore drill with canary, offline snapshot validation, RTO table | toutouwai |
| `runbooks/project-wind-down.md` | Shutting a project down without stranding mail, accounts, backups or bills | toutouwai |
| `templates/phase-spec.md` | Phase spec with Tier 1/Tier 2 and a verifiable "done when" | toutouwai |
| `templates/phase-dispatch-handoff.md` | State-of-the-world handoff for the next (AI or human) implementing session | toutouwai |

## Related notes (hubs)

- `~/notes/atomic/Dev/Kubernetes platform patterns.md`
- `~/notes/atomic/Dev/Backups & disaster recovery.md`
- `~/notes/atomic/Dev/CI-CD & release patterns.md`
- `~/notes/atomic/Dev/Recurring pitfalls & their remedies.md`
- Per-project lessons: `~/notes/atomic/Dev/{dml,toutouwai}-lessons-learned/`

See `BACKLOG.md` for what's still worth extracting.
