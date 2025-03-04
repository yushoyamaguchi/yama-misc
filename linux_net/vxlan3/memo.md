# fdb使う場合のencap
カーネルはinnerL2ヘッダのmacアドレスを見て、それに対応するvxlanのdst ipを探す。
これを実現するためには、vxlanのIPアドレスがすでにarpで解決されている必要がある。


# オーバーレイのマルチキャスト

```
bridge fdb append 00:00:00:00:00:00 dev vxlan0 dst 172.20.20.1 self
```
この設定を入れると、vxlan0のIPをinnerのsrc-ipに指定すれば、宛先macが```01:00:5e:00:00:01```のマルチキャストパケットが、outer-dst-ip=dst 172.20.20.1でカプセル化されて送信される。
00:00:00:00:00:00はunknow unicastを意味するマジックナンバー。
マルチキャストの場合はarp解決をしてfdbにエントリを追加するというフローが飛ばされるため、このunknouw unicastのエントリが参照される。