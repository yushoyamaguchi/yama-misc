#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-wg-dummy-int100-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash