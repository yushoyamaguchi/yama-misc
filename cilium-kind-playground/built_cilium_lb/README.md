# pre-requires
- kind
- kubectl
- docker
- clium-cli

## user setting
```
sudo usermod -aG docker $USER
```

# setup
```
export CILIUM_DIR=path/to/cilium
bash setup.sh
```

# delete
```
bash cleanup.sh
```