# ホストからLoadbalancer Serviceのexternal-ipにアクセスするために
Serviceのexternal-ipのアドレス帯をnodeのものとは異なるものにした上で、そのアドレスへのルーティング情報をホストのブリッジデバイスに設定する。
```bash
# 172.18.0.3はnodeのeth0のIPアドレス. node間のロードバランシングを直接したい場合はECMPを使う
sudo ip route add 172.121.0.0/16 via 172.19.0.3 dev br-af4e087aa09b
```