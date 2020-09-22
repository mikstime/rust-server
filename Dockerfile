FROM rustlang/rust:nightly
MAINTAINER Michael Balitsky

WORKDIR /usr/src/rust_server
COPY . .
RUN cargo install --path .
EXPOSE 80
CMD ["rust_server"]