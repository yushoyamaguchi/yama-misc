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
    linux-tools-generic \
    vim \
    tshark \
    libnl-utils \
    wireguard-tools \
    nginx  # Nginxをインストール

RUN rm -rf /var/lib/apt/lists/*

# Nginxのデフォルト設定ファイルを削除
RUN rm /etc/nginx/sites-enabled/default

# 単純なHTMLファイルを作成
RUN echo 'I am a nginx server.' > /var/www/html/index.html

# Nginxのカスタム設定ファイルを作成（メトリクス設定を含む）
RUN printf 'server {\n\
    listen 80;\n\
    server_name localhost;\n\
    \n\
    location / {\n\
        root /var/www/html;\n\
        index index.html index.htm;\n\
    }\n\
    \n\
    # Prometheus metrics endpoint\n\
    location /metrics {\n\
        stub_status on;\n\
        access_log off;\n\
        allow 127.0.0.1;\n\
        allow ::1;\n\
        deny all;\n\
    }\n\
}' > /etc/nginx/sites-available/default.conf

# /etc/nginx/sites-available/default.confを有効にするためのシンボリックリンクを作成
RUN ln -s /etc/nginx/sites-available/default.conf /etc/nginx/sites-enabled/default.conf

# ヘルスチェック用のスクリプトを追加
COPY healthcheck.sh /healthcheck.sh
RUN chmod +x /healthcheck.sh

HEALTHCHECK --interval=30s --timeout=3s \
    CMD /healthcheck.sh

# Nginxをフォアグラウンドで実行
CMD ["nginx", "-g", "daemon off;"]