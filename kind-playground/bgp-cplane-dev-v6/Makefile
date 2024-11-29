CILIUM_DIR ?= /home/y-yamaguchi/yusho/cilium


deploy-cluster:
	sudo echo "Deploying cluster"
	kind create cluster --config cluster.yaml
	sudo containerlab -t topo.yaml deploy

destroy:
	sudo containerlab -t topo.yaml destroy -c
	kind delete clusters bgp-cplane-dev-v6

install-cilium:
	make -C $(CILIUM_DIR) kind-image
	cilium install --wait --chart-directory=$(CILIUM_DIR)/install/kubernetes/cilium --values cilium-config.yaml

wait-for-cilium:
	kubectl -n kube-system wait --for=condition=Ready pod -l k8s-app=cilium --timeout=300s

apply-policy:
	kubectl apply -f bgpcc.yaml
	# In IPv6 single-stack, Cilium cannot pick router-id automatically.
	kubectl -n kube-system annotate node bgp-cplane-dev-v6-control-plane "cilium.io/bgp-virtual-router.65001=router-id=10.0.0.2"
	kubectl -n kube-system annotate node bgp-cplane-dev-v6-worker "cilium.io/bgp-virtual-router.65002=router-id=10.0.0.3"


setup: deploy-cluster install-cilium wait-for-cilium apply-policy

