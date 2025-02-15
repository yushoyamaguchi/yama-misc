FROM nicolaka/netshoot:latest


# Install pwru
RUN curl -LO https://github.com/cilium/pwru/releases/download/v1.0.9/pwru-linux-amd64.tar.gz && \
tar xzf pwru-linux-amd64.tar.gz && \
mv pwru /usr/local/bin/ && \
chmod +x /usr/local/bin/pwru && \
rm pwru-linux-amd64.tar.gz