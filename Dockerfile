FROM rust:1.73.0

WORKDIR /app
# Install the required system dependacies for our linking configuration
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./target/release/zero2prod" ]