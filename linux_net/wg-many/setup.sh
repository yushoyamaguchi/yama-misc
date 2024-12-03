#!/bin/bash

mkdir -p auto-gen

bash ./inventory_ini.sh 3


cp ./wg0_node.conf.j2 auto-gen/wg0_node.conf.j2
cp ./wg0_server.conf.j2 auto-gen/wg0_server.conf.j2

sudo containerlab -t auto-gen/lab.yaml deploy

ansible-playbook -i auto-gen/inventory.ini auto-gen/wireguard_setup.yml
