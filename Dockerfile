FROM rust:latest
ENV SRCDIR /usr/src/backend
ENV YEW_FULLSTACK_STATIC "/usr/local/share/yew-fullstack/www"
ENV YEW_FULLSTACK_HOST "0.0.0.0"
ENV YEW_FULLSTACK_PORT "8080"
ENV RUST_LOG "info"
WORKDIR ${SRCDIR}
COPY backend .
RUN cargo build --release
RUN cp /usr/src/backend/target/release/yew-fullstack-backend /usr/local/bin/yew-fullstack-backend
RUN rm -rf ${SRCDIR}
WORKDIR ${YEW_FULLSTACK_STATIC}
COPY frontend/dist/* ${YEW_FULLSTACK_STATIC}/
WORKDIR /usr/local/bin
EXPOSE ${YEW_FULLSTACK_PORT}
CMD ["/usr/local/bin/yew-fullstack-backend"]
