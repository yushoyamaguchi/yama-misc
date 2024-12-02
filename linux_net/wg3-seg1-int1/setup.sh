#!/bin/bash

sudo containerlab -t lab.yaml deploy

ansible-playbook -i inventory.ini wireguard_setup.yml
# ansible-playbook -i inventory.ini wireguard_setup_samekey.yml