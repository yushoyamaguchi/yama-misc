#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-bgp-cplane-dev-v4-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash