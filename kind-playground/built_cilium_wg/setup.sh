#!/bin/bash

# Default value
DEFAULT_CILIUM_DIR="/home/y-yamaguchi/yusho/cilium"

# Check if CILIUM_DIR is already set in the environment
if [ -z "${CILIUM_DIR}" ]; then
    echo "CILIUM_DIR is not set. Using default value."
    CILIUM_DIR="${DEFAULT_CILIUM_DIR}"
else
    echo "Using existing CILIUM_DIR from environment."
fi

echo "CILIUM_DIR is set to: $CILIUM_DIR"


echo "Setting up Cilium with WireGuard on Kind"

echo "building custom kind node image with network tools"
docker build --network=host -t custom-kind-node:v1.30.0-network-tools ./kind-image

echo "creating kind cluster"
kind create cluster --config=kind-config.yaml

echo "building cilium image"
make -C "${CILIUM_DIR}" kind-image-fast

echo "installing cilium"
make -C "${CILIUM_DIR}" kind-install-cilium-fast

cilium status --wait > /dev/null 2>&1

echo "applying cilium config"
cilium upgrade --values cilium-config.yaml

cilium status --wait > /dev/null 2>&1

echo "deploying netshoot pod"
kubectl apply -f deploy-netshoot.yaml