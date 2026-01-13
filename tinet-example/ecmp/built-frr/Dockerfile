FROM frr:centos-8-9b598c4b2e



# Install network-scripts (provides ifup / ifdown)
RUN dnf install -y network-scripts

# optional: clean cache
RUN dnf clean all