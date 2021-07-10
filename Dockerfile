FROM registry.access.redhat.com/ubi8/ubi
WORKDIR /fedora-coreos-pinger

COPY . .
RUN yum install -y gcc openssl-devel
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y -q
RUN echo "export PATH=\"$HOME/.cargo/bin:$PATH\"" > ~/.bashrc \
    && source ~/.bashrc \
    && cargo build
CMD [ "/root/.cargo/bin/cargo", "test" ]
