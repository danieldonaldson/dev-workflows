#!/usr/bin/env bash
# Shared Cilium install for every Talos cluster (local QEMU + Hetzner).
# CNI is "none" in the machine config, so nodes stay NotReady until this runs; the
# ArgoCD platform `cilium` Application later ADOPTS this release (same name/namespace/
# values), so the values here must equal what ArgoCD renders. Idempotent (helm
# upgrade --install). Two load-bearing, non-obvious values:
#   - kubeProxyReplacement=true + KubePrism (localhost:7445): Talos disables kube-proxy;
#     Cilium's eBPF replacement reaches the apiserver via KubePrism, not a ClusterIP.
#   - socketLB.hostNamespaceOnly=true: socket-LB does ClusterIP translation at connect()
#     time, but gVisor's sandboxed syscalls never reach the host cgroup hook, so runsc
#     agent pods can't resolve any ClusterIP (SYN goes to the untranslated VIP -> dropped
#     by egress policy as "world"). hostNamespaceOnly restricts socket-LB to the host
#     netns and pushes ClusterIP DNAT into the tc/veth datapath that gVisor traffic DOES
#     traverse; runc pods fall back to the same tc-LB with no loss. Origin: toutouwai k8s/scripts/install-cilium.sh.
#
# Per-caller deltas (env vars):
#   TUNNEL=vxlan          -> add routingMode=tunnel + tunnelProtocol=vxlan (node-to-node
#                            overlay on the private net).
#   OPERATOR_REPLICAS=<n> -> pin operator.replicas (single node: 1 — the operator's 2
#                            default replicas share the same host port, so the 2nd is
#                            Pending and `cilium status --wait` hangs). Bump when node 2 lands.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# CILIUM_VERSION: use the exported value if a caller already sourced talos.env, else load it.
if [ -z "${CILIUM_VERSION:-}" ]; then
  set -a && . "$SCRIPT_DIR/../talos.env" && set +a
fi

extra=()
if [ -n "${OPERATOR_REPLICAS:-}" ]; then
  extra+=(--set "operator.replicas=${OPERATOR_REPLICAS}")
fi
if [ "${TUNNEL:-}" = "vxlan" ]; then
  extra+=(--set routingMode=tunnel --set tunnelProtocol=vxlan)
fi

helm repo add cilium https://helm.cilium.io >/dev/null 2>&1 || true
helm repo update cilium >/dev/null
helm upgrade --install cilium cilium/cilium \
  --version "$CILIUM_VERSION" \
  --namespace kube-system \
  --set ipam.mode=kubernetes \
  --set kubeProxyReplacement=true \
  --set k8sServiceHost=localhost \
  --set k8sServicePort=7445 \
  --set cgroup.autoMount.enabled=false \
  --set cgroup.hostRoot=/sys/fs/cgroup \
  --set securityContext.capabilities.ciliumAgent="{CHOWN,KILL,NET_ADMIN,NET_RAW,IPC_LOCK,SYS_ADMIN,SYS_RESOURCE,DAC_OVERRIDE,FOWNER,SETGID,SETUID}" \
  --set securityContext.capabilities.cleanCiliumState="{NET_ADMIN,SYS_ADMIN,SYS_RESOURCE}" \
  --set resources.requests.memory=320Mi \
  --set socketLB.hostNamespaceOnly=true \
  --set prometheus.enabled=true \
  --set operator.prometheus.enabled=true \
  --set hubble.enabled=true \
  --set hubble.metrics.enabled="{dns,drop,tcp,flow,port-distribution,icmp,httpV2}" \
  ${extra[@]+"${extra[@]}"}
