PROJECT = demo

LIB = roll
SRC = $(PROJECT).c
HEADERS = $(LIB).h

STATIC_LIB = $(LIB_PATH)/lib$(LIB).a
SHARED_LIB = $(LIB_PATH)/lib$(LIB).so

DYLIB_BIN = $(PROJECT)-dynamic
SHARED_BIN = $(PROJECT)-shared
STATIC_BIN = $(PROJECT)-static

CFLAGS += -g -Wall -Werror -Wconversion -fno-builtin

# only used to link against shared lib
LDFLAGS = -L $(LIB_PATH) -l $(LIB)

PRODUCTS = $(DYLIB_BIN) $(SHARED_BIN) $(STATIC_BIN)
LIB_PATH = ./target/debug

# default args; override with `ARGS="2 6"`
ARGS = 3 8

#
# phony targets
#

.PHONY: all dynamic shared static run clean distclean

all: build dynamic shared static

build: $(DYLIB_BIN) $(SHARED_BIN) $(STATIC_BIN)

dynamic: $(SHARED_LIB) $(DYLIB_BIN)
	LD_LIBRARY_PATH="$(LIB_PATH)" ./$(DYLIB_BIN) $(ARGS)

shared: $(SHARED_BIN)
	LD_LIBRARY_PATH="$(LIB_PATH)" ./$(SHARED_BIN) $(ARGS)

static: $(STATIC_BIN)
	./$(STATIC_BIN) $(ARGS)

run: static shared dynamic

clean:
	rm -rf $(PRODUCTS)

distclean: clean
	cargo clean

#
# real targets
#

$(SHARED_LIB):
	cargo build --lib

$(STATIC_LIB):
	cargo build --lib

$(DYLIB_BIN): $(HEADERS) $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(DYLIB_BIN) -DDYNAMIC_ROLL

$(SHARED_BIN): $(HEADERS) $(SRC) $(SHARED_LIB)
	$(CC) $(CFLAGS) $(SRC) -o $(SHARED_BIN) $(LDFLAGS)

$(STATIC_BIN): $(HEADERS) $(SRC) $(STATIC_LIB)
	$(CC) $(CFLAGS) $(SRC) -o $(STATIC_BIN) $(STATIC_LIB)

