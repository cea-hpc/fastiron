ARG BASEIMAGE=rust:slim-bullseye

# BUILDER PATTERN

FROM $BASEIMAGE AS builder

WORKDIR /builder

COPY . .

RUN --mount=type=cache,target=/cargo CARGO_HOME=/cargo cargo install --path=fastiron --root /builder/install

# FINAL IMAGE

FROM debian:bullseye-slim

WORKDIR /fastiron

COPY --from=builder /builder/install/bin /usr/bin

COPY --from=builder /builder/input_files/ /fastiron/input_files/

COPY --from=builder /builder/scripts/ /fastiron/scripts/

CMD ["./scripts/test.sh"]
