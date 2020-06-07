FROM rust:latest
ENV SRCDIR /usr/src/backend
ENV YEW_FULLSTACK_STATIC "/usr/local/share/yew-fullstack/www"
ENV YEW_FULLSTACK_HOST "0.0.0.0"
ENV YEW_FULLSTACK_PORT "8080"
ENV RUST_LOG "info"
RUN cargo install cargo-watch
WORKDIR ${SRCDIR}
# really needed?
COPY . .
EXPOSE ${YEW_FULLSTACK_PORT}
CMD ["/usr/src/backend/scripts/run-dev.sh"]
