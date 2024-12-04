#!/bin/bash

sudo containerlab -t auto-gen/lab.yaml destroy

sudo ip route del 192.168.0.0/24