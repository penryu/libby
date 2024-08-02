PROJECT = roll

SRC = $(PROJECT).c
HEADERS = $(PROJECT).h

SHARED_BIN = $(PROJECT)-shared
SHARED_LIB = $(LIB_PATH)/libroll.so

STATIC_BIN = $(PROJECT)-static
STATIC_LIB = $(LIB_PATH)/libroll.a

DYLIB_BIN = $(PROJECT)-dynamic

PRODUCTS = $(SHARED_BIN) $(STATIC_BIN)
CFLAGS += -g -Wall -Wextra -Wconversion -pedantic -fno-builtin
LDFLAGS = -L $(LIB_PATH) -l roll

LIB_PATH = ./target/release

ARGS = 3 8

#
# primary targets
#

.PHONY: all
all: build

.PHONY: build
build: $(SHARED_LIB) $(DYLIB_BIN) $(SHARED_BIN) $(STATIC_BIN)

.PHONY: cargo-release
cargo-release:
	cargo build --lib --release

.PHONY: check
check: check-static check-shared check-dynamic

.PHONY: run
run: run-static run-shared run-dynamic

#
# check
#

.PHONY: check-dynamic check-shared check-static

check-dynamic: $(DYLIB_BIN)
	LD_LIBRARY_PATH="$(LIB_PATH)" ldd $(DYLIB_BIN)

check-shared: $(SHARED_BIN)
	LD_LIBRARY_PATH="$(LIB_PATH)" ldd $(SHARED_BIN)

check-static: $(STATIC_BIN)
	ldd $(STATIC_BIN)

#
# run
#

.PHONY: run-dynamic run-shared run-static

run-dynamic: $(DYLIB_BIN)
	LD_LIBRARY_PATH="$(LIB_PATH)" ./$(DYLIB_BIN) $(ARGS)

run-shared: $(SHARED_BIN)
	LD_LIBRARY_PATH="$(LIB_PATH)" ./$(SHARED_BIN) $(ARGS)

run-static: $(STATIC_BIN)
	./$(STATIC_BIN) $(ARGS)

.PHONY: clean distclean
clean:
	rm -rf $(PRODUCTS)
distclean: clean
	cargo clean

#
# real targets
#

$(SHARED_LIB): cargo-release

$(STATIC_LIB): cargo-release

$(DYLIB_BIN): $(SRC) $(HEADERS) $(SHARED_LIB)
	$(CC) -DDYNAMIC_ROLL $(CFLAGS) $(SRC) -o $(DYLIB_BIN)

$(SHARED_BIN): $(SRC) $(HEADERS) $(SHARED_LIB)
	$(CC) $(CFLAGS) $(SRC) -o $(SHARED_BIN) $(LDFLAGS)

$(STATIC_BIN): $(SRC) $(HEADERS) $(STATIC_LIB)
	$(CC) $(CFLAGS) $(SRC) -o $(STATIC_BIN) $(STATIC_LIB)

