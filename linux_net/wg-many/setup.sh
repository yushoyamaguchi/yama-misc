#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <number>"
  exit 1
fi

if ! [[ $1 =~ ^[0-9]+$ ]]; then
  echo "Error: Argument must be a number."
  exit 1
fi

if [ $1 -lt 2 ] || [ $1 -gt 50000 ]; then
  echo "Error: Number must be between 2 and 254."
  exit 1
fi

sudo pwd

mkdir -p auto-gen

bash ./inventory_ini.sh $1
bash ./generate_playbook.sh $1
bash ./generate_lab.sh $1


cp ./wg0_node.conf.j2 auto-gen/wg0_node.conf.j2
cp ./wg0_server.conf.j2 auto-gen/wg0_server.conf.j2

sudo containerlab -t auto-gen/lab.yaml deploy

sudo ip route add 172.29.0.0/16 via 172.20.20.3

ansible-playbook -i auto-gen/inventory.ini auto-gen/wireguard_setup.yml

THIRD_OCTET=$(( ($1 - 1) / 250 ))
FOURTH_OCTET=$(( ($1 - 1) % 250 + 1 ))

docker exec -it clab-wg-many-server1 ping -c 3 10.0.$THIRD_OCTET.$FOURTH_OCTET