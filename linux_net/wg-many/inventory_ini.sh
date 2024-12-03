#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <number>"
  exit 1
fi

if ! [[ $1 =~ ^[0-9]+$ ]]; then
  echo "Error: Argument must be a number."
  exit 1
fi

if [ $1 -lt 2 ] || [ $1 -gt 254 ]; then
  echo "Error: Number must be between 2 and 254."
  exit 1
fi

{
  echo "[wireguard_nodes]"
  echo "server1 ansible_connection=docker ansible_host=clab-wg-many-server1 ansible_remote_tmp=/tmp/.ansible/tmp"
  for i in $(seq 2 $1); do
    echo "node$i ansible_connection=docker ansible_host=clab-wg-many-node$i ansible_remote_tmp=/tmp/.ansible/tmp"
  done
} > auto-gen/inventory.ini