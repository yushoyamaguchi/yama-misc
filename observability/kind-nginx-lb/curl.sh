#!/bin/bash

for i in {1..10}
do
  curl 172.21.0.0 &
done

wait