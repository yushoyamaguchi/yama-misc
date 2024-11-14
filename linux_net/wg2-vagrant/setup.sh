#!/bin/bash

HOME_DIR="$HOME"

vagrant up

ssh-keygen -f "$HOME_DIR/.ssh/known_hosts" -R "192.168.56.31"
ssh-keygen -f "$HOME_DIR/.ssh/known_hosts" -R "192.168.56.32"

sleep 60

ansible-playbook -i inventory.ini wireguard_setup.yml
