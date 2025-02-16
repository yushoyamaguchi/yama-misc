# Linux NativeのvxlanデバイスでvxlanのinnerをVMのものにする方法
vxlanデバイスとtapを同じbridgeデバイスにアタッチすると、VMからのパケットをそのままvxlanデバイスから出せて、vxlanのinnerをVMのものにすることができる。
bridgeにnicを接続するとnicがそのbiridbeデバイスのポートの口となり、nicにipアドレスはなくなる。
つまり、vxlanデバイスとtapを同じbridgeデバイスにアタッチすると、vxlanデバイスもtapもipアドレスを持たなくなる。
また、tapの口から来たパケットがvxlan-devの口から出ていくように、bridge-devに対して設定しなければいけない。
これはfdbを使ってやる。
VNIごとにvxlanデバイスを用意する必要がある。

fdbの代わりにマルチキャストグループを使う方法もある

# マルチキャストとBUM
```
ip link add vxlan100 type vxlan id 100 dstport 4789 group 239.0.1.1 dev net0
```
linuxではこのような感じ
これでBUMも対応できる
これを入れると、結果的にfdbに適切なエントリが勝手に入ってる。