#!/bin/bash

# Default value
KIND_IMAGE="custom-kind-node:v1.29.0-network-tools2"
NETSHOOT_NGINX_IMAGE="my-netshoot-nginx:v1"


echo "Setting up Cilium with Geneve on Kind"

echo "building custom kind node image with network tools"
docker build --network=host -t $KIND_IMAGE ./kind-image

echo "building netshoot & nginx images"
docker build --network=host -t $NETSHOOT_NGINX_IMAGE ./pod-image

echo "creating kind cluster"
kind create cluster --image $KIND_IMAGE --config=kind-config.yaml

echo "loading netshoot & nginx images into kind cluster"
kind load docker-image $NETSHOOT_NGINX_IMAGE --name kind


echo "installing cilium"
cilium install --wait  --values cilium-config.yaml --version 1.16.0

echo "deploying prometheus"
helm install prometheus prometheus-community/kube-prometheus-stack \
  --create-namespace \
  --namespace monitoring


helm install fluent-bit-example fluent/fluent-bit \
  -n monitoring \
  -f values.yaml

echo "install loki"
helm install loki grafana/loki-stack -n monitoring

echo "deploying netshoot and nginx pods"
kubectl apply -f daemonsets.yaml
