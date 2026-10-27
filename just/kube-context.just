# Per-shell, fail-closed cluster selection. Import from a project justfile:
#     import? "path/to/kube-context.just"
# and provide an env file defining the per-env config paths, e.g. k8s/cluster.env:
#     KUBECONFIG_LOCAL=$HOME/.kube/myproj-local   TALOSCONFIG_LOCAL=$HOME/.talos/myproj-local
#     KUBECONFIG_PROD=$HOME/.kube/myproj-prod     TALOSCONFIG_PROD=$HOME/.talos/myproj-prod
#
# Usage:   eval "$(just ctx local)"      # or add:  tctx(){ eval "$(just ctx "$1")"; }
#          just ctx                      # show current selection
#
# Why: one file per env (never `talosctl config merge` — it renames contexts on collision and
# stale PKI shadows live clusters), selected per shell via env vars, so a fresh shell with
# nothing selected errors instead of acting on whatever was touched last.
# Notes: ~/notes/atomic/Dev/toutouwai-lessons-learned/Point each shell at a cluster explicitly so kubectl fails closed.md

cluster_env := "k8s/cluster.env"

ctx target="":
    #!/usr/bin/env bash
    set -euo pipefail
    set -a && . "{{cluster_env}}" && set +a
    case "{{target}}" in
      local) tc="${TALOSCONFIG_LOCAL:-}"; kc="$KUBECONFIG_LOCAL" ;;
      prod)  tc="${TALOSCONFIG_PROD:-}";  kc="$KUBECONFIG_PROD"  ;;
      "")    { echo "TALOSCONFIG=${TALOSCONFIG:-<unset>}"; echo "KUBECONFIG=${KUBECONFIG:-<unset>}";
               echo 'usage: eval "$(just ctx local|prod)"'; } >&2; exit 0 ;;
      *)     echo 'usage: eval "$(just ctx local|prod)"' >&2; exit 1 ;;
    esac
    [ -f "$kc" ] || { echo "!! $kc missing — bring {{target}} up first" >&2; exit 1; }
    KUBECONFIG="$kc" kubectl --request-timeout=10s get --raw /version >/dev/null 2>&1 \
      || { echo "!! $kc cannot reach its API server" >&2; exit 1; }
    if [ -n "$tc" ]; then
      [ -f "$tc" ] || { echo "!! $tc missing" >&2; exit 1; }
      # -e/-n explicitly: some talosconfigs carry an endpoint but no node.
      ep="$(TALOSCONFIG="$tc" talosctl config info 2>/dev/null | awk '/^Endpoints:/{print $2}')"
      TALOSCONFIG="$tc" timeout 10 talosctl -e "$ep" -n "$ep" version >/dev/null 2>&1 \
        || { echo "!! $tc cannot reach $ep" >&2; exit 1; }
      printf 'export TALOSCONFIG=%q\n' "$tc"
    fi
    echo ">> {{target}} reachable" >&2
    printf 'export KUBECONFIG=%q\n' "$kc"

# Guard for destructive recipes:  `just _require-ctx prod` as the first line.
_require-ctx env:
    #!/usr/bin/env bash
    set -euo pipefail
    set -a && . "{{cluster_env}}" && set +a
    want="$( [ "{{env}}" = prod ] && echo "$KUBECONFIG_PROD" || echo "$KUBECONFIG_LOCAL" )"
    [ "${KUBECONFIG:-}" = "$want" ] || { echo "!! this shell is not on {{env}} — run: tctx {{env}}" >&2; exit 1; }
