#!/usr/bin/env bash
# Regenerate SealedSecrets from their SOPS-encrypted sources.
#
# SOPS is the editable source of truth; SealedSecrets is how the clusters consume secrets.
# Each overlay that needs secrets holds a `secrets.sops.yaml` (SOPS-encrypted, committed) —
# a plain multi-document Secret manifest. This script decrypts it and, per document, runs
# kubeseal to write `sealed-secrets.yaml` beside it (the file the overlay's kustomization
# references). Nothing plaintext ever touches disk.
#
# Usage:
#   k8s/scripts/reseal.sh                         # reseal every secrets.sops.yaml under k8s/
#   k8s/scripts/reseal.sh k8s/infra/garage/overlays/local   # one overlay dir (or file)
#
# Requires: sops (with the age private key available), kubeseal. kubeseal fetches the
# controller's public cert from the current kube-context by default; set SEALED_SECRETS_CERT
# to a committed cert path to reseal offline / for a cluster you're not currently pointed at.
#
# Env:
#   SEALED_SECRETS_CERT       path to the controller's public cert (optional; else fetched live)
#   SEALED_SECRETS_NAMESPACE  controller namespace (default: kube-system)
#   SEARCH_ROOT               where a no-arg run looks for secrets.sops.yaml (default: <repo>/k8s)
set -euo pipefail

CONTROLLER_NS="${SEALED_SECRETS_NAMESPACE:-kube-system}"
REPO_ROOT="${REPO_ROOT:-$(git rev-parse --show-toplevel)}"

kubeseal_args=(--controller-namespace "$CONTROLLER_NS" --format yaml)
if [[ -n "${SEALED_SECRETS_CERT:-}" ]]; then
  kubeseal_args+=(--cert "$SEALED_SECRETS_CERT")
fi

# Resolve the list of source files from the args (dirs -> their secrets.sops.yaml).
sources=()
if [[ $# -eq 0 ]]; then
  while IFS= read -r f; do sources+=("$f"); done \
    < <(find "${SEARCH_ROOT:-$REPO_ROOT/k8s}" -name secrets.sops.yaml -type f | sort)
else
  for arg in "$@"; do
    if [[ -d "$arg" ]]; then
      sources+=("$arg/secrets.sops.yaml")
    else
      sources+=("$arg")
    fi
  done
fi

if [[ ${#sources[@]} -eq 0 ]]; then
  echo "No secrets.sops.yaml sources found." >&2
  exit 1
fi

for src in "${sources[@]}"; do
  if [[ ! -f "$src" ]]; then
    echo "!! missing source: $src" >&2
    exit 1
  fi
  dir="$(dirname "$src")"
  out="$dir/sealed-secrets.yaml"
  echo ">> resealing $src -> $out"

  tmp_out="$(mktemp)"
  trap 'rm -f "$tmp_out"' EXIT
  first=1

  # Decrypt first so a sops failure (missing/expired age key) aborts loudly instead of
  # being swallowed by the process substitution and misreported as "no Secret documents".
  decrypted="$(sops -d "$src")" || {
    echo "!! sops decrypt failed for $src (age key available?)" >&2
    exit 1
  }

  # Split on YAML document separators (---), feeding each Secret document to kubeseal.
  # Null-delimited so multi-line values survive.
  while IFS= read -r -d '' doc; do
    # skip blank / comment-only documents
    [[ -z "${doc//[[:space:][:cntrl:]]/}" ]] && continue
    if [[ $first -eq 0 ]]; then printf -- '---\n' >>"$tmp_out"; fi
    printf '%s\n' "$doc" | kubeseal "${kubeseal_args[@]}" >>"$tmp_out"
    first=0
  done < <(printf '%s' "$decrypted" | awk '
      /^---[[:space:]]*$/ { if (buf != "") { printf "%s%c", buf, 0; buf="" } ; next }
      { buf = buf $0 "\n" }
      END { if (buf != "") printf "%s%c", buf, 0 }
    ')

  if [[ $first -eq 1 ]]; then
    echo "!! no Secret documents found in $src" >&2
    rm -f "$tmp_out"; trap - EXIT
    exit 1
  fi

  mv "$tmp_out" "$out"
  trap - EXIT
  echo "   wrote $(grep -c '^kind: SealedSecret' "$out") SealedSecret(s)"
done

echo ">> done. Uncomment sealed-secrets.yaml in the overlay kustomization(s) if new, then commit + sync."
