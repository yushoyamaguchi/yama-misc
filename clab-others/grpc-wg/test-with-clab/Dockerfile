# 基本イメージとしてubuntu:latestを使用
FROM ubuntu:noble


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
    wireguard-tools

RUN rm -rf /var/lib/apt/lists/*