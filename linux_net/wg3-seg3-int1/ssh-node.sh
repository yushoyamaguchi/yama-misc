#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-wg3-seg3-int1-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash