#!/bin/bash
sudo pwd
docker build --network=host -t netshoot-pwru .
sudo containerlab -t lab.yaml deploy