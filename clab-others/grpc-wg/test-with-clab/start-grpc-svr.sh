#!/bin/bash
/usr/local/bin/grpc-svr > /var/log/grpc-svr.log 2>&1 &
echo "gRPC server started in background"
exit 0