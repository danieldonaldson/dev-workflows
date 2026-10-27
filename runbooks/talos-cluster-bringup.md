# Talos cluster bring-up (cloud, single node → GitOps)

Ordered runbook distilled from toutouwai's `k8s/hcloud/README.md` (Hetzner + OpenTofu + Packer
+ talhelper + Cilium + ArgoCD). **[1x]** = once, **[every]** = every (re)build, **[$]** = spends money.

| # | Step | Notes |
|---|------|-------|
| 1 | Build the Talos snapshot with Packer **[1x per Talos minor] [$]** | Image Factory schematic ID pins system extensions (gVisor, iscsi-tools). Reuse the same ID on every `talosctl upgrade` or extensions vanish. |
| 2 | Refresh firewall admin CIDR → `tofu init/plan/apply` **[$]** | State in S3 with versioning + `use_lockfile`. Inbound 80/443 **only from your CDN's ranges** if one fronts the origin. |
| 3 | `talhelper gensecret` → SOPS-encrypt **[1x]** | `trap` to delete plaintext on failure. Same PKI across rebuilds = existing configs keep working. |
| 4 | Render talconfig with the node IP from tofu output → genconfig | Don't pin etcd `advertisedSubnets` to a private CIDR where the IP is a /32. |
| 5 | `apply-config --insecure --mode=reboot` | First apply from maintenance mode **must reboot**, or etcd half-pivots and wedges later. |
| 6 | `talosctl bootstrap` → fetch kubeconfig **into a per-env file** | Never `config merge` into a shared file (context renames). |
| 7 | Install Cilium (`k8s/install-cilium-talos-gvisor.sh`) | Node is NotReady until CNI lands (CNI=none). `socketLB.hostNamespaceOnly=true` for gVisor. Single node: `operator.replicas=1`. |
| 8 | Verify: `talosctl health`, node Ready, gVisor pod `/proc/version`, a `toFQDNs` policy enforcing | Make this a recipe. |
| 9 | Approve the pending kubelet-serving CSR | Otherwise logs/exec/port-forward fail `tls: internal error`; ArgoCD adds `kubelet-csr-approver` after. |
| 10 | Install ArgoCD + repo credential (from SOPS) | Plain Secret, not sealed — ArgoCD must read the repo before the sealing controller exists. |
| 11 | **Restore the pinned Sealed Secrets key** | **Before** step 12. Reverse it and every committed SealedSecret is dead. |
| 12 | Apply the root app-of-apps; wait for all Synced/Healthy | Require N consecutive all-green polls (children appear over waves). |
| 13 | Out-of-band secrets checklist | Image pull secret from host `docker login` — the one manual step. Everything else should be in git. |

## Done when

- `tofu plan` is empty against live; `packer build` reproduces the snapshot from a clean checkout.
- Every ArgoCD app Synced/Healthy; smoke recipes (gvisor, storage, netpol, admission) green.
- An etcd snapshot CronJob exists **before** the node holds anything you care about.
- `delete_protection` + `rebuild_protection` + `prevent_destroy` set on a node you intend to keep.
