#
# Iron Guard Server
#
# @author Afaan Bilal
# @link   https://afaan.dev
#

#
# Stage 1 (Build)
#

FROM rust:1.67-slim-buster AS build

WORKDIR /iron-guard-server

COPY . .

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:buster-slim

WORKDIR /iron-guard-server

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y pkg-config libssl-dev
COPY --from=build /iron-guard-server/target/release/iron-guard-server ./iron-guard-server

EXPOSE 8000

# And away we go...
CMD [ "./iron-guard-server" ]
