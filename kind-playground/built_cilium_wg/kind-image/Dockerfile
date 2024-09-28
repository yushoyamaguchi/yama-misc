# Use the official Kind node image as base
FROM kindest/node:v1.29.0

RUN echo 'deb http://deb.debian.org/debian bookworm main contrib non-free' > /etc/apt/sources.list
RUN echo 'deb http://deb.debian.org/debian-security bookworm-security main contrib non-free' >> /etc/apt/sources.list
RUN echo 'deb http://deb.debian.org/debian bookworm-updates main contrib non-free' >> /etc/apt/sources.list



# Update and install network tools
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
    bpftool \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install some additional useful tools
RUN apt-get update && apt-get install -y \
    vim \
    htop \
    strace \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

