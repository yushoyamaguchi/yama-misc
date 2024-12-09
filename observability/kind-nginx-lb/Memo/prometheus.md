# prometheusの概要
基本的には、メトリクスを取ってくる役割のexporterと、そのメトリクスを収集して可視化する役割の本体の2つの役割がある。
exporterは、メトリクスを取る対象によって種類がたくさんある。
- node-exporter: ホストのメトリクスを取得する
- blackbox-exporter: ネットワークのメトリクスを取得する
- mysqld-exporter: MySQLのメトリクスを取得する
- nginx-exporter: Nginxのメトリクスを取得する
- cadvisor: コンテナのメトリクスを取得する (exporterみたいな使い方ができる)

# デプロイ方法
## kubernetesクラスタで使う場合
本体は、クラスタにインストールするとdaemonsetのように動作する。(ciliumでいうoperatorやagentみたいなイメージ)
exporterは、Podとしてデプロイする。 メトリクスを見たいコンテナがいるpodの別のコンテナとしてデプロイすることもある。

## ローカルで使う場合
exporterと本体をローカルにインストールして、起動する。 systemdとかで起動するのが一般的？

# grafanaとの連携
HTTP APIを使って、prometheusのメトリクスをgrafanaに表示することができる。
APIサーバの提供はprometheus本体の仕事
(grafanaが、prometheusが提供するAPIのデータフォーマットに対応するための実装を持っている。)

## grafanaのデプロイ
```
helm install prometheus prometheus-community/kube-prometheus-stack
```
これでインストールしたら、grafanaもセットで入ってる。

## grafana確認方法
```
kubectl get secret -n monitoring prometheus-grafana -o jsonpath="{.data.admin-password}" | base64 --decode ; echo
```
これでgrafanaのパスワードがわかる。
```
kubectl port-forward -n monitoring svc/prometheus-grafana 3000:80
```
これでgrafanaのポートフォワードができる。

sshで開発してるときは、ssh元で見たい。
そんな時は
```
# ローカルマシンのターミナルで実行
ssh -L 3000:localhost:3000 username@remote-host
```
ホスト側の別ターミナルでこれを実行しておけば、3000ポートにアクセスすると、ssh先開発マシンの3000ポートにアクセスできる。

