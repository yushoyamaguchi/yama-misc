#!/bin/bash

set -euo pipefail

KUBE_SYSTEM_NAMESPACE="kube-system"

CILIUM_NAMESPACE="${KUBE_SYSTEM_NAMESPACE}"
CILIUM_VERSION="1.15.4"
CLUSTER_NAME_PREFIX="cl"
CLUSTER_1_NAME="${CLUSTER_NAME_PREFIX}1"
CLUSTER_1_CONTEXT="kind-${CLUSTER_1_NAME}"
CLUSTER_2_NAME="${CLUSTER_NAME_PREFIX}2"
CLUSTER_2_CONTEXT="kind-${CLUSTER_2_NAME}"



function info() {
    echo "=> ${1}"
}



info "Deploying the test application..."
kubectl apply -f ./deploy-nginx1.yaml
