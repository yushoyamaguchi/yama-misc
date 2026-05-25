#!/bin/bash

# Default value
KIND_IMAGE="custom-kind-node:v1.29.0-network-tools2"
NETSHOOT_NGINX_IMAGE="my-netshoot-nginx:v2"

sudo pwd

echo "building cni-plugins"
(cd ../../../cni-plugins && sudo bash build_linux.sh)

echo "building route-override-cni"
(cd ../../../route-override-cni && sudo bash build_linux.sh)

echo "building custom kind node image with network tools"
docker build --network=host -t $KIND_IMAGE ./kind-image

echo "building netshoot & nginx images"
docker build --network=host -t $NETSHOOT_NGINX_IMAGE ./pod-image

echo "creating kind cluster"
kind create cluster --image $KIND_IMAGE --config=kind-config.yaml

echo "setup for multus"
ansible-playbook -i inventory.ini create-br-ecmp.yml
kubectl apply -f https://raw.githubusercontent.com/k8snetworkplumbingwg/multus-cni/master/deployments/multus-daemonset.yml

kind load docker-image $NETSHOOT_NGINX_IMAGE --name kind

kubectl apply -f nad.yaml
kubectl apply -f daemonsets.yaml
