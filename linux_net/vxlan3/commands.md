# fdb状態確認
bridge fdb show
bridge fdb show dev vxlan0

# マルチキャスト
socat STDIO UDP4-RECVFROM:6666,ip-add-membership=239.255.0.1:10.0.0.2
socat STDIO UDP4-RECVFROM:6666,ip-add-membership=239.255.0.1:10.0.0.3

echo "Test multicast message" | socat - UDP-DATAGRAM:239.255.0.1:6666,ip-multicast-ttl=10,bind=10.0.0.1