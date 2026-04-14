KIND_IMAGE="custom-kind-node:v1.29.0-network-tools2"
NETSHOOT_NGINX_IMAGE="my-netshoot-nginx:v2"


echo "creating kind cluster"
kind create cluster --image $KIND_IMAGE --config=kind-config.yaml

echo "loading netshoot & nginx images into kind cluster"
kind load docker-image $NETSHOOT_NGINX_IMAGE --name kind

echo "copy cni binary to every node"
cd load