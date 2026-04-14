# ECMP Static Default Route Lab

Containerlab topology for testing how Linux behaves when two static default routes are configured in parallel. The lab stands up four `nicolaka/netshoot` containers wired so the `client` node can reach the `internet` node through two equal-cost gateways.

## Topology

```
        10.0.1.0/30              192.0.2.0/30
client -------------- gw1 -------------------- internet
   \
    \
     \
      10.0.2.0/30            192.0.2.4/30
       -------------- gw2 -------------------- internet
```

- `client`: place where you configure the pair of default routes.
- `gw1`/`gw2`: simple Linux routers that forward traffic toward the `internet` node.
- `internet`: represents a remote destination (`203.0.113.1/32`) reachable via both gateways; it also holds return routes to the client-side networks.

## Addressing

| Link | client | gw1/gw2 side | internet side |
| ---- | ------ | ------------ | ------------- |
| client ↔ gw1 | `10.0.1.2/30` | `10.0.1.1/30` | – |
| client ↔ gw2 | `10.0.2.2/30` | `10.0.2.1/30` | – |
| gw1 ↔ internet | – | `192.0.2.1/30` | `192.0.2.2/30` |
| gw2 ↔ internet | – | `192.0.2.5/30` | `192.0.2.6/30` |
| internet loopback | – | – | `203.0.113.1/32` |

## Usage

```bash
sudo containerlab deploy -t clab-ecmp_static.yaml
# ... run your tests ...
sudo containerlab destroy -t clab-ecmp_static.yaml -c
```

Enter the client container and add the candidate default routes. Example using the Linux `nexthop` syntax:

```bash
docker exec -it clab-ecmp_static-client bash -lc \
  "ip route replace default \\n     nexthop via 10.0.1.1 dev eth1 weight 1 \\n     nexthop via 10.0.2.1 dev eth2 weight 1"
```

To mimic two independent `ip route add default via ...` statements instead:

```bash
docker exec -it clab-ecmp_static-client bash -lc "ip route replace default via 10.0.1.1 dev eth1"
docker exec -it clab-ecmp_static-client bash -lc "ip route append default via 10.0.2.1 dev eth2"
```

Test reachability toward the shared destination:

```bash
docker exec -it clab-ecmp_static-client bash -lc "ping -I 10.0.1.2 203.0.113.1"
docker exec -it clab-ecmp_static-client bash -lc "ping -I 10.0.2.2 203.0.113.1"
```

Use `ip route show`, `ip -s route get`, or packet captures (e.g., `tcpdump -i eth1`) inside the nodes to verify whether traffic is being hashed across both next-hops.
