#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-vxlan-3nodes-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash