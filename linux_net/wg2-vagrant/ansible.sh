#!/bin/bash

HOME_DIR="$HOME"
CURRENT_DIR=$(pwd)


ansible-playbook -i inventory.ini wireguard_setup.yml
