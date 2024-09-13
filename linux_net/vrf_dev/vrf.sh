#!/bin/bash

# VRFの名前
VRF="vrf1"

# ネットワークネームスペースの名前
NS1="netns1"
NS2="netns2"

# vethペアの名前
VETH1_HOST="veth1_host"
VETH1_NS="veth1_ns"
VETH2_HOST="veth2_host"
VETH2_NS="veth2_ns"

# IPアドレス
IP1="10.1.1.1/24" # ネームスペース1内のveth
IP2="10.1.2.1/24" # ネームスペース2内のveth
HOST_IP1="10.1.1.254/24" # ホスト側veth1
HOST_IP2="10.1.2.254/24" # ホスト側veth2

HOST_IP1_ONLY="10.1.1.254" # ホスト側veth1
HOST_IP2_ONLY="10.1.2.254" # ホスト側veth2

create_setup() {
    sysctl -w net.ipv4.ip_forward=1

    iptables -P FORWARD ACCEPT
    iptables -A FORWARD -i $VRF -o $VRF -j ACCEPT

    # VRFデバイスの作成
    ip link add $VRF type vrf table 1
    ip link set $VRF up

    # VRFルールの作成
    ip rule add table 1

    # ネットワークネームスペースの作成
    ip netns add $NS1
    ip netns add $NS2

    # vethペアの作成
    ip link add $VETH1_HOST type veth peer name $VETH1_NS
    ip link add $VETH2_HOST type veth peer name $VETH2_NS

    # vethをネームスペースに移動
    ip link set $VETH1_NS netns $NS1
    ip link set $VETH2_NS netns $NS2

    # vethをVRFに接続
    ip link set $VETH1_HOST master $VRF
    ip link set $VETH2_HOST master $VRF

    # インターフェースを有効化
    ip link set $VETH1_HOST up
    ip link set $VETH2_HOST up
    ip netns exec $NS1 ip link set $VETH1_NS up
    ip netns exec $NS2 ip link set $VETH2_NS up

    # ループバックインターフェースの有効化
    ip netns exec $NS1 ip link set lo up
    ip netns exec $NS2 ip link set lo up

    # IPアドレスの設定（ホスト側）
    ip addr add $HOST_IP1 dev $VETH1_HOST
    ip addr add $HOST_IP2 dev $VETH2_HOST

    # IPアドレスの設定（ネームスペース側）
    ip netns exec $NS1 ip addr add $IP1 dev $VETH1_NS
    ip netns exec $NS2 ip addr add $IP2 dev $VETH2_NS

    # デフォルトルートを設定
    ip netns exec $NS1 ip route add default via $HOST_IP1_ONLY
    ip netns exec $NS2 ip route add default via $HOST_IP2_ONLY

    echo "Setup completed successfully."
}

cleanup() {
    # ネットワークネームスペースの削除
    ip netns del $NS1
    ip netns del $NS2

    # vethペアの削除（自動的に両端が削除される）
    ip link del $VETH1_HOST
    ip link del $VETH2_HOST

    # VRFの削除
    ip link del $VRF

    # VRFルールの削除
    ip rule del table 1

    echo "Cleanup completed successfully."
}

case "$1" in
    create)
        create_setup
        ;;
    cleanup)
        cleanup
        ;;
    *)
        echo "Usage: $0 {create|cleanup}"
        exit 1
        ;;
esac

exit 0
