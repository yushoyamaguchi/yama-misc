#!/bin/bash

docker build -t ssh-bg .

sudo containerlab -t lab.yaml deploy

