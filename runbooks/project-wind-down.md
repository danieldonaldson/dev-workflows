# Project wind-down checklist

From shutting down toutouwai (2026-08-25). Copy into the repo as `docs/wind-down/README.md`
and keep **Done** / **Still to do** tables — the record outlives your memory of it.
Notes: `~/notes/atomic/Dev/toutouwai-lessons-learned/Winding down a project is a migration — export before you delete.md`

## Before deleting anything

- [ ] **Export DNS** (BIND + JSON) for every zone you'll touch; write a restore script beside it.
      Check what *else* the zone serves (mail: MX/SPF/DKIM/DMARC/MTA-STS; tunnels; other apps).
- [ ] Note the SOA negative TTL — if you might restore records, lower it first.
- [ ] List every third-party account **registered to an address on this domain**. Close or
      re-home them *before* MX goes, or password resets bounce.
- [ ] Commit and push the working tree.

## Compute & data

- [ ] **Delete** (not power off) servers — stopped servers still bill. Delete snapshots not under IaC.
- [ ] Sweep the cloud account: volumes, IPs, LBs, networks, firewalls, certs, SSH keys.
- [ ] Note whether instance types are still orderable at the old price (rebuild cost).
- [ ] Object-Locked buckets: set a minimum-delay lifecycle rule so they drain; calendar the date
      the last lock expires. Do **not** re-apply IaC that re-arms retention.
- [ ] Delete the IaC **state bucket last** — it's the only record of what exists.
- [ ] Local: remove dead kube/talos contexts (back configs up first); stop VMs, containers, forwards.

## Credentials & integrations

- [ ] Revoke API tokens — especially any scoped wider than the project (e.g. all DNS zones).
- [ ] Payments: cancel live subscriptions, deactivate products (webhooks died with the cluster).
- [ ] Uptime monitors (or they alert on a dead origin forever).
- [ ] OAuth clients, bots, email sending identities, CAPTCHA widgets, alert webhooks.
- [ ] Remaining credit balances at API providers.
- [ ] Container registry packages (needs `read:packages`/`delete:packages`) — optional; cheapest thaw path.

## Repo

- [ ] Record known drift (failing lints, IaC that no longer matches live) so a thaw starts honestly.
- [ ] Archive the GitHub repo (stops Actions, Dependabot).
- [ ] Extract lessons into the notes vault before the context fades.
