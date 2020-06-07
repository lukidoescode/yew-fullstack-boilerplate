FROM node:latest
ENV SRCDIR /usr/src/frontend
ENV RUST_LOG "info"
WORKDIR /root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install wasm-pack
WORKDIR ${SRCDIR}
COPY . .
RUN /usr/local/bin/yarn install
EXPOSE 8000
CMD ["/usr/local/bin/yarn", "run", "start:dev"]