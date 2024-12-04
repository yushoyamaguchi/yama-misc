#!/bin/bash

sudo containerlab -t auto-gen/lab.yaml destroy

sudo ip route del 172.29.200.0/24