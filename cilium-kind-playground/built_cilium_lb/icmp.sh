#!/bin/bash



for i in {1..10}
do
  sudo hping3 --udp --dontfrag -d 140 -c 1 -p 55555 172.121.0.0 &
done

wait
