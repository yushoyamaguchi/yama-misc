#!/bin/bash

NODE_SUFFIX=$1

NODE_NAME="clab-json-wg-grpc-${NODE_SUFFIX}"

docker exec -it ${NODE_NAME} bash