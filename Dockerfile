FROM rustlang/rust:nightly
MAINTAINER Michael Balitsky

WORKDIR /usr/src/rust_server
COPY . .
RUN cargo install --path .
EXPOSE 3000
CMD ["rust_server"]