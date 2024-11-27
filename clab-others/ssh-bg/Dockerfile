FROM rastasheep/ubuntu-sshd:latest

RUN apt-get update && \
    apt-get install -y openssh-server sudo tcpdump net-tools iproute2 && \
    apt-get clean