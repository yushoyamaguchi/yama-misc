CILIUM_DIR ?= /home/y-yamaguchi/yusho/cilium

build-kind-image:
	sudo pwd
	docker build -t custom-kind-node:v1.29.0-network-tools ./kind-image

deploy-cluster:
	sudo echo "Deploying cluster"
	kind create cluster --config cluster.yaml
	sudo containerlab -t topo.yaml deploy

deploy-cluster-dup:
	sudo echo "Deploying cluster"
	kind create cluster --config cluster.yaml
	sudo containerlab -t topo_dup.yaml deploy

destroy:
	sudo containerlab -t topo.yaml destroy -c
	kind delete clusters bgp-cplane-dev-v4

install-cilium:
	make -C $(CILIUM_DIR) kind-image
	cilium install --wait --chart-directory=$(CILIUM_DIR)/install/kubernetes/cilium --values cilium-dev.yaml

install-cilium-dev:
	make -C $(CILIUM_DIR) kind-image
	cilium install --wait --chart-directory=$(CILIUM_DIR)/install/kubernetes/cilium --values cilium-config-dev.yaml

wait-for-cilium:
	kubectl -n kube-system wait --for=condition=Ready pod -l k8s-app=cilium --timeout=300s

apply-policy:
	kubectl apply -f bgpcc.yaml -f bgpadv.yaml

apply-policy-dup:
	kubectl apply -f bgpcc-dup.yaml -f bgpadv.yaml

netshoot-daemonset:
	kubectl apply -f netshoot-ds.yaml


setup: build-kind-image deploy-cluster install-cilium wait-for-cilium apply-policy netshoot-daemonset

dev-setup: build-kind-image deploy-cluster install-cilium-dev wait-for-cilium apply-policy netshoot-daemonset


dup-setup: build-kind-image deploy-cluster-dup install-cilium wait-for-cilium apply-policy-dup netshoot-daemonset

bgp-override:
	kubectl apply -f bgp-override.yaml



NAMESPACE=default
WORKER_NODE=bgp-cplane-dev-v4-worker
CONTROL_PLANE_NODE=bgp-cplane-dev-v4-control-plane
POD_LABEL=app=netshoot

ping-w-c:
	WORKER_POD=$$(kubectl get pods -n $(NAMESPACE) -l $(POD_LABEL) -o jsonpath='{.items[?(@.spec.nodeName=="$(WORKER_NODE)")].metadata.name}'); \
	CONTROL_PLANE_POD=$$(kubectl get pods -n $(NAMESPACE) -l $(POD_LABEL) -o jsonpath='{.items[?(@.spec.nodeName=="$(CONTROL_PLANE_NODE)")].metadata.name}'); \
	CONTROL_PLANE_IP=$$(kubectl get pod $$CONTROL_PLANE_POD -n $(NAMESPACE) -o jsonpath='{.status.podIP}'); \
	kubectl exec -n $(NAMESPACE) $$WORKER_POD -- ping -c 3 -w 4 $$CONTROL_PLANE_IP

ping-c-w:
	WORKER_POD=$$(kubectl get pods -n $(NAMESPACE) -l $(POD_LABEL) -o jsonpath='{.items[?(@.spec.nodeName=="$(WORKER_NODE)")].metadata.name}'); \
	CONTROL_PLANE_POD=$$(kubectl get pods -n $(NAMESPACE) -l $(POD_LABEL) -o jsonpath='{.items[?(@.spec.nodeName=="$(CONTROL_PLANE_NODE)")].metadata.name}'); \
	WORKER_IP=$$(kubectl get pod $$WORKER_POD -n $(NAMESPACE) -o jsonpath='{.status.podIP}'); \
	kubectl exec -n $(NAMESPACE) $$CONTROL_PLANE_POD -- ping -c 3 -w 4 $$WORKER_IP

check-bgp-info-in-cilium:
	kubectl get ciliumbgpnodeconfigs -o yaml