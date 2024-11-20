FROM ubuntu:jammy

# Install dependencies
RUN apt -y update && \
    apt -y install \
    iproute2 \
    iputils-ping \
    net-tools \
    tcpdump \
    iperf \
    socat \
    curl \
    gnupg2 \
    lsb-release

# Add FRR repository
RUN curl -s https://deb.frrouting.org/frr/keys.gpg | apt-key add - && \
    echo deb https://deb.frrouting.org/frr $(lsb_release -s -c) frr-10 | tee -a /etc/apt/sources.list.d/frr.list

# Install latest FRR
RUN apt update && apt -y install frr frr-pythontools

# Enable PIM
RUN sed -i -e 's/pimd=no/pimd=yes/g' /etc/frr/daemons