#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-mcast-frr-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash