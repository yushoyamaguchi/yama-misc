#!/bin/bash

# ARPエントリを追加する関数
add_arp_entry() {
    local source_container="$1"  # ARPエントリを追加するコンテナ名
    local target_container="$2"  # MACアドレスを取得したいコンテナ名
    local ip_addr="$3"          # 追加するARPエントリのIPアドレス

    # 両方のコンテナの存在確認
    if ! docker ps -q -f name="^${source_container}$" > /dev/null; then
        echo "Error: Source container ${source_container} does not exist or is not running"
        return 1
    fi

    if ! docker ps -q -f name="^${target_container}$" > /dev/null; then
        echo "Error: Target container ${target_container} does not exist or is not running"
        return 1
    fi

    # ターゲットコンテナからMACアドレスを取得
    local mac_addr=$(docker exec "$target_container" ip link show net0 | \
                    grep -oE "([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}" | head -n1)

    if [ -z "$mac_addr" ]; then
        echo "Error: Could not get MAC address for interface net0 in container ${target_container}"
        return 1
    fi

    # ソースコンテナでARPエントリを追加
    if ! docker exec "$source_container" ip neigh add "$ip_addr" lladdr "$mac_addr" dev net0; then
        echo "Error: Failed to add ARP entry in container ${source_container}"
        return 1
    fi

    echo "Successfully added ARP entry in ${source_container} for ${ip_addr} with MAC ${mac_addr} from ${target_container}"
    return 0
}

# 使用例：
add_arp_entry "C3" "C1" "10.100.1.1"
add_arp_entry "C1" "C3" "10.100.2.3"
add_arp_entry "C4" "C2" "10.100.1.2"
add_arp_entry "C2" "C4" "10.100.2.4"