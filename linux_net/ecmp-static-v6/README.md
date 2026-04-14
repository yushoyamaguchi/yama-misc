# ECMP Static Default Route Lab (IPv6)

IPv6 版の Containerlab トポロジ。Linux が 2 本の静的デフォルトルートを並列設定したときの挙動を検証する。`nicolaka/netshoot` コンテナ 4 台で構成し、`client` ノードが 2 台の等コストゲートウェイを経由して `internet` ノードに到達できる。

## Topology

```
        fd00:1::/126             fd00:3::/126
client -------------- gw1 -------------------- internet
   \
    \
     \
      fd00:2::/126           fd00:4::/126
       -------------- gw2 -------------------- internet
```

- `client`: ECMP デフォルトルートを設定する起点ノード
- `gw1`/`gw2`: `internet` ノードへパケットを転送するシンプルな Linux ルータ
- `internet`: 両ゲートウェイ経由で到達可能なリモート宛先 (`2001:db8::1/128`) を持つノード

## Addressing

| Link | client | gw1/gw2 side | internet side |
| ---- | ------ | ------------ | ------------- |
| client ↔ gw1 | `fd00:1::2/126` | `fd00:1::1/126` | – |
| client ↔ gw2 | `fd00:2::2/126` | `fd00:2::1/126` | – |
| gw1 ↔ internet | – | `fd00:3::1/126` | `fd00:3::2/126` |
| gw2 ↔ internet | – | `fd00:4::1/126` | `fd00:4::2/126` |
| internet loopback | – | – | `2001:db8::1/128` |

## Usage

```bash
sudo containerlab deploy -t clab-ecmp-static-v6.yaml
# ... テスト実施 ...
sudo containerlab destroy -t clab-ecmp-static-v6.yaml -c
```

`client` コンテナに入り、ECMP デフォルトルートを追加する。`nexthop` 構文を使う例:

```bash
docker exec -it clab-ecmp_static_v6-client bash -lc \
  "ip -6 route replace default \
     nexthop via fd00:1::1 dev eth1 weight 1 \
     nexthop via fd00:2::1 dev eth2 weight 1"
```

2 本の `ip route add` で追加する場合:

```bash
docker exec -it clab-ecmp_static_v6-client bash -lc "ip -6 route replace default via fd00:1::1 dev eth1"
docker exec -it clab-ecmp_static_v6-client bash -lc "ip -6 route append default via fd00:2::1 dev eth2"
```

疎通確認:

```bash
docker exec -it clab-ecmp_static_v6-client bash -lc "ping6 -I fd00:1::2 2001:db8::1"
docker exec -it clab-ecmp_static_v6-client bash -lc "ping6 -I fd00:2::2 2001:db8::1"
```

`ip -6 route show`、`ip -6 route get`、`tcpdump -i eth1` などを使ってトラフィックが両ネクストホップにハッシュ分散されているか確認する。
