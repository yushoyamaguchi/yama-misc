# サブネットマスクについて
WireGuardデバイスはL3デバイスなので、そこに割り振られているサブネットマスクにはL2セグメントという意味はない。
そのため、同じアドレス帯の異なるノード間のパケットをL3ルーティングするという、直感に反したことができる。
サブネットマスクはルーティングにおいて経路をまとめて扱うためのものという以上の意味はない。
