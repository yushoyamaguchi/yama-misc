#!/bin/bash

sudo pwd

IMAGE_NAME="yamaguchi-frr:mcast1"

docker build -t $IMAGE_NAME .

sudo containerlab -t lab.yaml deploy