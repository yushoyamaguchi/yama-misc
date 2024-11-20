#!/bin/bash

sudo pwd

IMAGE_NAME="yamaguchi-frr:mcast1"
LATEST_IMAGE_NAME="yamaguchi-frr-latest:mcast1"

#docker build -t $IMAGE_NAME .
docker build -t $LATEST_IMAGE_NAME ./latest-frr

sudo containerlab -t lab.yaml deploy