#!/bin/bash

sudo containerlab -t auto-gen/lab.yaml destroy

sudo ip route del 172.29.0.0/16