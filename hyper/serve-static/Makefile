SHELL := $(shell which bash)
SELF  := $(patsubst %/,%,$(dir $(abspath $(firstword $(MAKEFILE_LIST)))))

PACKAGE_NAME := serve-static
PACKAGE_LIB  := lib$(subst -,_,$(PACKAGE_NAME))
PACKAGE_BIN  := $(PACKAGE_NAME)-bin

RUST_LOG       := debug
RUST_BACKTRACE := full

HOST_UID ?= $(shell id -u)
HOST_GID ?= $(shell id -g)

define DOCKERFILE

FROM ubuntu:18.04

RUN apt-get -q update \
 && DEBIAN_FRONTEND=noninteractive apt-get -q install -y \
    bash \
    curl \
    musl-dev \
    musl-tools

ENV PATH=/root/.cargo/bin:$$PATH

RUN curl --proto "=https" --tlsv1.2 -Sf https://sh.rustup.rs \
    | bash -s -- -y --default-toolchain stable-x86_64-unknown-linux-gnu \
 && rustup target add x86_64-unknown-linux-musl

WORKDIR /build/

COPY Cargo.toml Cargo.lock ./
RUN install -d ./src/ \
 && echo "" >./src/lib.rs \
 && echo "fn main() {}" >./src/bin.rs \
 && cargo build \
    --target x86_64-unknown-linux-musl \
    --release \
 && find ./target/x86_64-unknown-linux-musl/ -name "$(PACKAGE_LIB)*" -o -name "$(PACKAGE_BIN)*" \
    | xargs rm -rf

COPY /src/ ./src/
RUN cargo build \
    --target x86_64-unknown-linux-musl \
    --release \
 && strip --strip-unneeded ./target/x86_64-unknown-linux-musl/release/$(PACKAGE_BIN)

VOLUME /target/

ENTRYPOINT []
CMD install -d /target/static/ \
 && cp ./target/x86_64-unknown-linux-musl/release/$(PACKAGE_BIN) /target/static/ \
 && exec chown -R $(HOST_UID):$(HOST_GID) /target/

endef

export

.PHONY: all t test b build d debug c clean

all: build

t test:
	cd $(SELF)/ && cargo test -- --nocapture

b build:
	cd $(SELF)/ && cargo build

d debug: build
	cd $(SELF)/ && ./target/debug/$(PACKAGE_BIN)

c clean:
	rm -rf $(SELF)/target/

.PHONY: s static

s static: $(wildcard $(SELF)/Cargo.*) $(wildcard $(SELF)/src/*.rs)
	docker build -t $(PACKAGE_NAME)-builder -f- $(SELF)/ <<< "$$DOCKERFILE"
	docker run -v $(SELF)/target/:/target/ --rm -t $(PACKAGE_NAME)-builder
