#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-grpc-wg-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash