#!/bin/bash

set -euo pipefail

KUBE_SYSTEM_NAMESPACE="kube-system"

CILIUM_NAMESPACE="${KUBE_SYSTEM_NAMESPACE}"
CILIUM_VERSION="1.15.4"
CLUSTER_NAME_PREFIX="cl"
CLUSTER_1_NAME="${CLUSTER_NAME_PREFIX}1"
CLUSTER_1_CONTEXT="kind-${CLUSTER_1_NAME}"



function info() {
    echo "=> ${1}"
}

info "Building kind image..."
docker build -t custom-kind-node:v1.29.0-network-tools ./cl1/kind-image


info "Creating the clusters..."
kind create cluster --name "${CLUSTER_1_NAME}" --config "${CLUSTER_1_NAME}/kind.yaml"

info "Building and loading the docker images..."
docker build -t simple-web1 ./cl1
kind load docker-image simple-web1 --name "${CLUSTER_1_NAME}"

info "Installing Cilium...."
cilium install --version "${CILIUM_VERSION}" \
    --set ipam.mode=kubernetes \
    --set hubble.ui.enabled=true \
    --set hubble.relay.enabled=true \
    --set kubeProxyReplacement=strict \
    --set bgpControlPlane.enabled=true \
    --set k8s.requireIPv4PodCIDR=true \
    --set externalIPs.enabled=true

cilium status --wait > /dev/null 2>&1    


