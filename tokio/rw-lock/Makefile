SHELL := $(shell which bash)
SELF  := $(patsubst %/,%,$(dir $(abspath $(firstword $(MAKEFILE_LIST)))))

PACKAGE_NAME := rw-lock
PACKAGE_BIN  := $(PACKAGE_NAME)-bin

RUST_LOG       := debug
RUST_BACKTRACE := full

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
