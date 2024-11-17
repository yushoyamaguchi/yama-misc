FROM ubuntu:jammy
RUN apt -y update
RUN apt -y install iproute2 iputils-ping net-tools tcpdump iperf socat frr
RUN sed -i -e 's/pimd=no/pimd=yes/g' /etc/frr/daemons