# DR rehearsal — wipe and restore, measured

Template from toutouwai's two production rehearsals (July 2026: 77 min → 17½ min RTO, four
latent bugs found). Run it **before** real traffic — that window is the only free rehearsal.
Notes: `~/notes/atomic/Dev/Backups & disaster recovery.md`

## 0. Preconditions (verify every one — a missing one discovered mid-flight is an outage)

- [ ] Offline root key (age) present and prints the expected public key: `age-keygen -y <key>`
- [ ] Sealing-controller key backup exists offline
- [ ] Cluster/machine secrets decrypt: `sops -d <talsecret> >/dev/null`
- [ ] Off-site store has a base backup per DB **and** a recent control-plane snapshot (list with versions shown)
- [ ] Repo credential for GitOps is in SOPS (no pasting during recovery)
- [ ] What prod tracks (branch) contains the digest pins you expect to come back

## 1. Validate the snapshot offline (non-destructive — do first)

Fetch → decrypt with the offline key → `etcdutl snapshot status --write-out=table`. A truncated
file fails `invalid database`. Expect keys in the thousands and size ≈ live DB size.
The decrypted snapshot holds every Secret in plaintext: own admin box only; `shred -u` after.

## 2. Capture pre-state, plant a canary, force fresh backups

- Row counts for a few stable tables; `get app -o wide` (all Synced/Healthy?).
- Insert a canary row **after** the last base backup; confirm its WAL segment reached off-site.
- Trigger a fresh control-plane snapshot; validate *that* object (step 1) before proceeding.
- If taking an on-demand DB base backup: **suspend the off-site mirror** until it completes.

## 3. Wipe — start the clock

`talosctl reset --graceful=false --reboot --system-labels-to-wipe STATE,EPHEMERAL`
Never the default `--wipe-mode all` (kills the boot partition; rebuild may be blocked by protection).

## 4. Restore

- **Branch A (primary): rebuild from git.** Bring-up runbook steps 4–13. DBs come back via
  `bootstrap: recovery` automatically.
- **Branch B (once, to prove the object): restore control-plane snapshot.** Expect stale
  objects pointing at wiped volumes; operators will *report healthy*. Don't trust it.

## 5. Verify & record

| Milestone | Elapsed |
|---|---|
| root app applied | |
| all DBs recovered | |
| all apps Synced/Healthy | |
| off-site mirror green again | |

- [ ] Canary row present, on the new timeline
- [ ] Row counts match step 2
- [ ] Backups/mirror running again
- [ ] RTO + every surprise recorded in the plan doc (an unrecorded rehearsal gets repeated)
- [ ] Fix each surprise in git, then **re-run** to prove the fixes converge unattended

## Not covered by this drill (say so explicitly)

Cloud layer (image build, IaC create, new IPs), per-tenant volume data, DNS/edge cutover.
