prepare-config:
	talosctl gen config talos-demo https://talos-controlplane:6443 --with-kubespan --config-patch @cluster.yaml --force


create-cluster:
	talosctl cluster create --name talos-demo --workers 2


delete-cluster:
	talosctl cluster destroy --name talos-demo



setup: prepare-config create-cluster