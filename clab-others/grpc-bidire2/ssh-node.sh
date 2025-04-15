#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-grpc-bidire2-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash