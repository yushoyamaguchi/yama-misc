#!/bin/bash

for i in {1..10}
do
  curl [fd00:1::] &
done

wait