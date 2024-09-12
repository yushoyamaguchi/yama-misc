SUDO ?= sudo

.PHONY: kind-image
kind-image:
	$(SUDO) ls > /dev/null 2>&1
	docker build -t custom-kind-node:v1.29.0-network-tools ./kind-image
	

.PHONY: start
start: kind-image
	$(SUDO) sysctl -w fs.inotify.max_user_instances=512
	$(SUDO) sysctl -w fs.inotify.max_user_watches=65536

	@echo "CREATE THE KIND CLUSTER"
	kind create cluster --config=kind-config.yaml

	@echo "CREATE TOPOLOGY"
	$(SUDO) containerlab -t lab.yaml deploy

	@echo "INSTALL CILIUM"
	cilium install --namespace kube-system --values cilium-config.yaml
	cilium status --wait > /dev/null 2>&1

	kubectl apply -f deploy-netshoot.yaml


.PHONY: stop
stop:
	kubectl delete -f deploy-netshoot.yaml
	$(SUDO) containerlab -t lab.yaml destroy
	kind delete cluster