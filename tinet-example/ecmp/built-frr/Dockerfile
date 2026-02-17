FROM frr:centos-8-333cba65c4

# network scripts
RUN dnf install -y network-scripts

# Network debugging tools (CentOS 8 minimal compatible set)
RUN dnf install -y \
    iproute \
    iputils \
    ethtool \
    tcpdump \
    traceroute \
    mtr \
    net-tools \
    nmap-ncat \
    socat \
    telnet \
    bind-utils \
    dhcp-client \
    ipcalc \
    perf \
    procps-ng \
    lsof \
    strace \
    wget \
    curl \
    rsync \
    vim \
    less \
    jq \
    python3 \
    wireshark-cli \
    which \
    hostname \
    pciutils \
    usbutils \
    iptables \
    nftables

# optional but useful and available
RUN dnf install -y \
    bpftool \
    elfutils \
    elfutils-libelf

# clean cache
RUN dnf clean all
