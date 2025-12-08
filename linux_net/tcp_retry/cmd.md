# ncサーバ node-a
nc -l -k -p 5000

# ncクライアント node-b
while true; do nc 172.18.0.2 5000; sleep 1; done

# iptables node-a
iptables -A INPUT -s 172.18.0.3 -p tcp --dport 5000 -j DROP

iptables -L INPUT --line-numbers
iptables -D INPUT 1

