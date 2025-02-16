#!/bin/bash

tinet reconf -c topo.yaml | sudo sh -x

bash ./add_arp_entry.sh