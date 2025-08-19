#!/bin/bash

# Default value
DEFAULT_CILIUM_DIR="/home/yusho/dev/cilium"
KIND_IMAGE="custom-kind-node:v1.29.0-network-tools2"
NETSHOOT_NGINX_IMAGE="my-netshoot-nginx:v1"

sudo pwd

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

echo "building netshoot & nginx images"
docker build --network=host -t $NETSHOOT_NGINX_IMAGE ./pod-image

echo "creating kind cluster"
sudo kind create cluster --image $KIND_IMAGE --config=kind-config.yaml

echo "loading netshoot & nginx images into kind cluster"
kind load docker-image $NETSHOOT_NGINX_IMAGE --name kind

echo "building cilium image"
make -C "${CILIUM_DIR}" kind-image

echo "installing cilium"
sudo cilium install --wait --chart-directory="${CILIUM_DIR}"/install/kubernetes/cilium --values cilium-config-v6-vip.yaml

sudo kubectl -n kube-system wait --for=condition=Ready pod -l k8s-app=cilium --timeout=300s

echo "deploying netshoot and nginx pods"
sudo kubectl apply -f daemonsets_v6_vip.yaml