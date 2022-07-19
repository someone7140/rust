FROM rust:1.60.0 AS build-stage

RUN USER=root cargo new --bin event-api
RUN apt-get update \
    && apt-get install -y -q \
    ca-certificates \
    locales \
    gnupg \
    apt-transport-https\
    libssl-dev \
    pkg-config \
    build-essential \
    git \
    wget \
    && echo "ja_JP UTF-8" > /etc/locale.gen  \
    && locale-gen

WORKDIR /event-api
COPY ./ .
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim AS production

RUN apt-get update  \
    && apt-get install -y -q \
    curl

# Copy build file
COPY --from=build-stage /event-api/target/release/event-api .
# Copy other file
COPY --from=build-stage /event-api/.env.prod .
WORKDIR /asset
COPY --from=build-stage /event-api/asset/ .

WORKDIR /
CMD ["./event-api"]
