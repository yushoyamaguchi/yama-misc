# ホストからLoadbalancer Serviceのexternal-ipにアクセスするために
Serviceのexternal-ipのアドレス帯をnodeのものとは異なるものにした上で、そのアドレスへのルーティング情報をホストのブリッジデバイスに設定する。
```bash
# 172.18.0.3はnodeのeth0のIPアドレス. node間のロードバランシングを直接したい場合はECMPを使う
sudo ip -6 route add fd00:1::/64 via fc00:f853:ccd:e793::4 dev br-99b0ec5828d4
```