FROM rust:1.51-slim-buster
ENV BUILD_DIR /home/build
RUN mkdir -p $BUILD_DIR
WORKDIR $BUILD_DIR
RUN --mount=type=cache,target=/var/cache/apt --mount=type=cache,target=/var/lib/apt \
      apt update \
      && apt upgrade -y \
      && apt install -y --no-install-recommends \
          git \
          libssl-dev \
          pkg-config
COPY . .
RUN cargo install --path .