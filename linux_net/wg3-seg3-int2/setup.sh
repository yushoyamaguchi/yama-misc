#!/bin/bash

sudo containerlab -t lab.yaml deploy

ansible-playbook -i inventory.ini wireguard_setup.yml