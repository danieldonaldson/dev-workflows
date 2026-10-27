# Reusable just building blocks. Import with:  import? "path/to/common.just"

set dotenv-load := false   # secrets come from SOPS on demand, never a .env file

# Supply-chain policy: refuse npm packages younger than 24h. Use {{pnpm}} in recipes and tell
# agents (CLAUDE.md) to call the just recipes rather than pnpm directly.
pnpm := "env PNPM_CONFIG_MINIMUM_RELEASE_AGE=1440 pnpm"

default:
    @just --list

# Refuse to run when the tree is dirty — for anything that deploys the COMMITTED tree
# (GitOps bootstrap, release tagging), where uncommitted work would silently not ship.
_require-clean:
    #!/usr/bin/env bash
    if [ -n "$(git status --porcelain)" ]; then
      echo "!! uncommitted changes — this deploys the committed tree; commit or stash first:" >&2
      git status --short >&2; exit 1
    fi

# Interactive confirm for destructive recipes; FORCE=1 skips it (CI/scripts).
_confirm msg:
    #!/usr/bin/env bash
    [ "${FORCE:-}" = 1 ] && exit 0
    read -r -p "{{msg}} [y/N] " a; [[ "$a" =~ ^[Yy]$ ]] || { echo aborted >&2; exit 1; }

# Run a command with SOPS-decrypted env vars, without writing plaintext anywhere.
#   just with-secrets secrets/hcloud/secrets.sops.yaml 'tofu -chdir=infra plan'
with-secrets file cmd:
    sops exec-env {{quote(file)}} {{quote(cmd)}}

# Recipe hygiene checklist (from a security pass over a 3,000-line justfile):
#  - quote() every user-supplied argument; pass values to psql as :'var', never interpolated
#  - feed tokens via stdin / env-file, never argv (visible in ps and shell history)
#  - umask 077 around temp credential files; trap 'rm -f' them on failure
#  - destructive recipes: explicit env argument + _require-ctx + _confirm
