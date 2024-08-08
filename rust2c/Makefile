# A heavily commented Makefile.
#
# I've tried to make the logic fairly straightforward,
# and tried to add comments whenever it isn't.

PROJECT = rust2c

# default args; override with `ARG="42"`
ARG ?= 1337

TARGET_PATH := target/debug

MAIN_SRC := src/main.rs

DLOPEN_MP_SRC := src/mp_dlopen.rs
DLOPEN_BIN_NAME := $(PROJECT)-dlopen
DLOPEN_BIN_PATH := $(TARGET_PATH)/$(DLOPEN_BIN_NAME)

SHARED_MP_SRC := src/mp_shared.rs
SHARED_BIN_NAME := $(PROJECT)-shared
SHARED_BIN_PATH := $(TARGET_PATH)/$(SHARED_BIN_NAME)

PRODUCTS := $(DLOPEN_BIN_PATH) $(SHARED_BIN_PATH)


#
# phony targets
#

.PHONY: all build distclean dlopen shared

all: check

build: $(SHARED_BIN_PATH) $(DLOPEN_BIN_PATH)

clean:
	cargo clean

distclean: clean


# Makes all binaries and runs them with the value of $(ARG) as argument.
run: run-shared run-dlopen

run-shared: $(SHARED_BIN_PATH)
	$(SHARED_BIN_PATH) $(ARG)

run-dlopen: $(DLOPEN_BIN_PATH)
	$(DLOPEN_BIN_PATH) $(ARG)


# Runs tests and tracks coverage. Coverage data is written to stdout.
test: test-shared test-dlopen

test-shared:
	cargo tarpaulin -o Stdout --features shared

test-dlopen:
	cargo tarpaulin -o Stdout --features dlopen

# real targets

$(SHARED_BIN_PATH): $(SHARED_MP_SRC) $(MAIN_SRC)
	cargo build --bin $(SHARED_BIN_NAME) --features shared

$(DLOPEN_BIN_PATH): $(DLOPEN_MP_SRC) $(MAIN_SRC)
	cargo build --bin $(DLOPEN_BIN_NAME) --features dlopen

