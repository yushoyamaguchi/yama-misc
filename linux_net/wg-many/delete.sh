#!/bin/bash

sudo containerlab -t auto-gen/lab.yaml destroy

sudo ip route del 192.168.200.0/24