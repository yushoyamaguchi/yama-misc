#!/bin/bash

for i in {1..100000}
do
  curl 172.121.0.0
  sleep 1
done

wait