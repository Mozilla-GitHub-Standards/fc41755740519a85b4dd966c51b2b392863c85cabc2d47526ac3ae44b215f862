# Docker 17.05 or higher required for multi-stage builds
FROM rust:1.30.0-stretch as builder
ADD . /app
WORKDIR /app
RUN \
    apt-get -qq update && \
    apt-get -qq install -y default-libmysqlclient-dev && \
    \
    cargo --version && \
    rustc --version && \
    mkdir -m 755 bin && \
    cargo build --release && \
    cp /app/target/release/pushbox /app/bin


FROM debian:stretch-slim
# FROM debian:stretch  # for debugging docker build
MAINTAINER <src+pushbox@jrconlin.com>
RUN \
    groupadd --gid 10001 app && \
    useradd --uid 10001 --gid 10001 --home /app --create-home app && \
    \
    apt-get -qq update && \
    apt-get -qq install -y default-libmysqlclient-dev libssl-dev ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists

COPY --from=builder /app/bin /app/bin
COPY --from=builder /app/version.json /app

WORKDIR /app
USER app

# override rocket's dev env defaulting to localhost
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_LOG off

CMD ["/app/bin/pushbox"]
