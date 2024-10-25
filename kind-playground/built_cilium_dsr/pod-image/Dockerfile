# 基本イメージとしてubuntu:latestを使用
FROM ubuntu:latest

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    iputils-ping \
    net-tools \
    iproute2 \
    tcpdump \
    nmap \
    iperf3 \
    mtr \
    traceroute \
    curl \
    wget \
    dnsutils \
    ethtool \
    iftop \
    nftables \
    bridge-utils \
    conntrack \
    socat \
    telnet \
    linux-tools-$(uname -r) \
    tshark \
    libnl-utils \
    wireguard-tools \
    nginx  # Nginxをインストール

RUN rm -rf /var/lib/apt/lists/*

# Nginxのデフォルト設定ファイルを削除
RUN rm /etc/nginx/sites-enabled/default

# 単純なHTMLファイルを作成
RUN echo 'I am a nginx server.' > /var/www/html/index.html

# Nginxのカスタム設定ファイルを作成
RUN printf 'server {\n\tlisten 80;\n\tserver_name localhost;\n\n\tlocation / {\n\t\troot /var/www/html;\n\t\tindex index.html index.htm;\n\t}\n}' > /etc/nginx/sites-available/default.conf

# /etc/nginx/sites-available/default.confを有効にするためのシンボリックリンクを作成
RUN ln -s /etc/nginx/sites-available/default.conf /etc/nginx/sites-enabled/default.conf

# Nginxをフォアグラウンドで実行
CMD ["nginx", "-g", "daemon off;"]