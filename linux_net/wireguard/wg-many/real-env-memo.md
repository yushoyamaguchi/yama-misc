# 実機で動かすときの方針
nodes側は性能のいいマシンで、今まで通りcontainerlabでdockerコンテナを立ててやる。
server側は、性能検証したい実機にsshdを入れて、containerがいるホストマシンからsshできるようにする。

# setup時にやるべきこと
- server側から各コンテナへのパケットを、コンテナホストマシンでルーティングするような設定を入れる。
- inventory.iniのserver1へのssh方法を、実機のものに変更する。