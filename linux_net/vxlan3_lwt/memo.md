# arp
lwtの設定入れるだけではarpが通らないので、arpの設定も必要
```
ip neigh add 10.0.0.2 lladdr aa:bb:cc:dd:ee:ff dev vxlan0
```
ただ、この設定が入ってても、vxlan0デバイスまでは行くが、dst-ipに適切な物理デバイスからパケットが出ていかない


# pwruでの分析結果
```
0xffff9f0c9c08ff00 11  <empty>:2158561  4026532686 0            vxlan0:2     0x0800 1500  98    10.0.0.1:0->10.0.0.2:0(icmp) kfree_skb_reason(SKB_DROP_REASON_NOT_SPECIFIED) vxlan_xmit[vxlan]
```

vxlan_xmitでdropされている。
vxlan fdbにエントリがないとdropされる処理があったので、それが原因かも

vxlan_xmitはeBPFでvxlan-devのtx-queueにリダイレクトした時には通らない処理なので、Ciliumはそれでこの問題を回避しているっぽい