FROM rust

WORKDIR /usr/src/tp
COPY . .

# Install zeromq
RUN echo "deb http://download.opensuse.org/repositories/network:/messaging:/zeromq:/release-stable/Debian_9.0/ ./" >> /etc/apt/sources.list
RUN wget https://download.opensuse.org/repositories/network:/messaging:/zeromq:/release-stable/Debian_9.0/Release.key -O- | apt-key add
RUN apt-get update
RUN yes | apt-get install libzmq5

# Install ProtoBuffers
RUN yes | apt-get install protobuf-compiler

RUN cargo install --path .

CMD ["tp"]
