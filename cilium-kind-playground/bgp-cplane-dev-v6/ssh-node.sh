#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-bgp-cplane-dev-v6-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash