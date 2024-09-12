#!/bin/bash

docker build -t custom-kind-node:v1.29.0-network-tools ./kind-image

kind create cluster --config=kind-config.yaml

cilium install --namespace kube-system --config cilium-config.yaml

cilium status --wait > /dev/null 2>&1

kubectl apply -f deploy-netshoot.yaml