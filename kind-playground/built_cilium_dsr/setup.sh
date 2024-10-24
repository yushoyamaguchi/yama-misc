#!/bin/bash

# Default value
DEFAULT_CILIUM_DIR="/home/y-yamaguchi/yusho/cilium"
KIND_IMAGE="custom-kind-node:v1.29.0-network-tools2"

# Check if CILIUM_DIR is already set in the environment
if [ -z "${CILIUM_DIR}" ]; then
    echo "CILIUM_DIR is not set. Using default value."
    CILIUM_DIR="${DEFAULT_CILIUM_DIR}"
else
    echo "Using existing CILIUM_DIR from environment."
fi

echo "CILIUM_DIR is set to: $CILIUM_DIR"


echo "Setting up Cilium with Geneve on Kind"

echo "building custom kind node image with network tools"
docker build --network=host -t $KIND_IMAGE ./kind-image

echo "creating kind cluster"
kind create cluster --image $KIND_IMAGE --config=kind-config.yaml

echo "building cilium image"
make -C "${CILIUM_DIR}" kind-image

echo "installing cilium"
cilium install --wait --chart-directory="${CILIUM_DIR}"/install/kubernetes/cilium --values cilium-config.yaml



echo "deploying netshoot pod"
kubectl apply -f deploy-netshoot.yaml