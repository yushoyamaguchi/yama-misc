# pre-requires
- kind
- kubectl
- docker
- clium-cli

## user setting
```
sudo usermod -aG docker $USER
```

## routing setting
```
sudo ip route add 172.121.0.0/16 via <kind-node-addr> dev <bridge device name which can communicate with kind node>
```

example:
```
sudo ip route add 172.121.0.0/16 via 172.18.0.3 dev br-2c205c7fa0f9 
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