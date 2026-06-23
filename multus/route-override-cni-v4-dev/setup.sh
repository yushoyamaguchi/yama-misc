#!/bin/bash

# Default value
KIND_IMAGE="custom-kind-node:v1.29.0-network-tools2"
NETSHOOT_NGINX_IMAGE="my-netshoot-nginx:v2"

sudo pwd

echo "building cni-plugins"
(cd ../../../cni-plugins && sudo bash build_linux.sh)

echo "building cni-route-override"
(cd ../../../cni-route-override && sudo bash build_linux.sh)

if ! docker image inspect "$KIND_IMAGE" >/dev/null 2>&1; then
    echo "building custom kind node image with network tools"
    docker build --network=host -t "$KIND_IMAGE" ./kind-image
else
    echo "$KIND_IMAGE already exists. Skip."
fi

if ! docker image inspect "$NETSHOOT_NGINX_IMAGE" >/dev/null 2>&1; then
    echo "building netshoot & nginx images"
    docker build --network=host -t "$NETSHOOT_NGINX_IMAGE" ./pod-image
else
    echo "$NETSHOOT_NGINX_IMAGE already exists. Skip."
fi

echo "creating kind cluster"
kind create cluster --image $KIND_IMAGE --config=kind-config.yaml

echo "setup for multus"
ansible-playbook -i inventory.ini create-br-ecmp.yml
kubectl apply -f https://raw.githubusercontent.com/k8snetworkplumbingwg/multus-cni/master/deployments/multus-daemonset.yml

kind load docker-image $NETSHOOT_NGINX_IMAGE --name kind

kubectl apply -f nad.yaml
kubectl wait --for=condition=Ready pod -n kube-system -l app=multus --timeout=120s

kubectl apply -f daemonsets.yaml
