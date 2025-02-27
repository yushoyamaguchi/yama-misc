#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-hub-spoke-seg1-int1-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash