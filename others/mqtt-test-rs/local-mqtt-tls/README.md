# certification
```bash
# CAキーとCA証明書の生成
openssl genrsa -out ca.key 2048
openssl req -new -x509 -days 365 -key ca.key -out ca.crt

# サーバーキーとCSRの生成
openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr

# サーバー証明書の生成
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 365
```


# mosquitto
## mosquitto.conf
/etc/mosquitto/mosquitto.conf
```conf
listener 8883
cafile /path/to/ca.crt
certfile /path/to/server.crt
keyfile /path/to/server.key
```

## restart mosquitto
```bash
sudo systemctl restart mosquitto
```

# 備考
ca.crtは、クライアント側でも使用する