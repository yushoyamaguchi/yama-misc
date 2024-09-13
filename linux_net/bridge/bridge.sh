#!/bin/bash

# ブリッジの名前
BRIDGE_NAME="br1"

# ネットワークネームスペースの名前
NS1="netns1"
NS2="netns2"

# vethペアの名前
VETH1_HOST="veth1_host"
VETH1_NS="veth1_ns"
VETH2_HOST="veth2_host"
VETH2_NS="veth2_ns"

# IPアドレス
IP1="192.168.1.1/24"
IP2="192.168.1.2/24"

create_setup() {
    sysctl -w net.ipv4.ip_forward=1

    iptables -P FORWARD ACCEPT
    iptables -A FORWARD -i $BRIDGE_NAME -o $BRIDGE_NAME -j ACCEPT

    # ブリッジの作成
    ip link add name $BRIDGE_NAME type bridge
    ip link set $BRIDGE_NAME up

    # ネットワークネームスペースの作成
    ip netns add $NS1
    ip netns add $NS2

    # vethペアの作成
    ip link add $VETH1_HOST type veth peer name $VETH1_NS
    ip link add $VETH2_HOST type veth peer name $VETH2_NS

    # vethをネームスペースに移動
    ip link set $VETH1_NS netns $NS1
    ip link set $VETH2_NS netns $NS2

    # vethをブリッジに接続
    ip link set $VETH1_HOST master $BRIDGE_NAME
    ip link set $VETH2_HOST master $BRIDGE_NAME

    # インターフェースを有効化
    ip link set $VETH1_HOST up
    ip link set $VETH2_HOST up
    ip netns exec $NS1 ip link set $VETH1_NS up
    ip netns exec $NS2 ip link set $VETH2_NS up

    # ループバックインターフェースの有効化
    ip netns exec $NS1 ip link set lo up
    ip netns exec $NS2 ip link set lo up

    # IPアドレスの設定
    ip netns exec $NS1 ip addr add $IP1 dev $VETH1_NS
    ip netns exec $NS2 ip addr add $IP2 dev $VETH2_NS

    echo "Setup completed successfully."
}

cleanup() {
    # ネットワークネームスペースの削除
    ip netns del $NS1
    ip netns del $NS2

    # vethペアの削除（自動的に両端が削除される）
    ip link del $VETH1_HOST
    ip link del $VETH2_HOST

    # ブリッジの削除
    ip link del $BRIDGE_NAME

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