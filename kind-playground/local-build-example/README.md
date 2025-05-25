# prerequisites
https://docs.cilium.io/en/stable/contributing/development/dev_setup/#verifying-your-development-setup

In cilium top directory, run:
```
make dev-doctor
```
If some modules are missing, install them.

# setup
```
CILIUM_DIR=/path/to/cilium bash ./setup.sh
```

# cleanup
```
bash ./cleanup.sh
```

# confirming debug-print
## debug-print from Go code
```
kubectl -n kube-system logs -l k8s-app=cilium -c cilium-agent | grep <keyword-in-debug-print>
```

## debug-print from eBPF code
### printk()
```
sudo bash ./show_bpf_printk.sh
```

### cilium_debug()
```
cilium-dbg monitor --verbose -t debug | grep <keyword-in-debug-print>
```