#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-vxlan-3nodes-lwt-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash