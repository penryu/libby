PROJECT = cdemo

# default args; override with `ARGS="2 6"`
ARGS ?= 3 8

# system-specific setup
SYSNAME := $(shell uname -s)

ifeq ($(SYSNAME),FreeBSD)
	LDFLAGS += -l pthread
endif

ifeq ($(SYSNAME),Darwin)
	SHARED_LIB_EXT = dylib
else
	SHARED_LIB_EXT = so
endif

SRC := $(PROJECT).c

LIB_PATH := ../target/debug
LIB := roll
LIB_H := $(LIB).h
LIB_FILE := lib$(LIB)
STATIC_LIB = $(LIB_PATH)/$(LIB_FILE).a
SHARED_LIB = $(LIB_PATH)/$(LIB_FILE).$(SHARED_LIB_EXT)

DYLIB_BIN := $(PROJECT)-dynamic
SHARED_BIN := $(PROJECT)-shared
STATIC_BIN := $(PROJECT)-static

PRODUCTS := $(DYLIB_BIN) $(SHARED_BIN) $(STATIC_BIN)

# Try to keep our C safe.
CFLAGS += -g -O0 -Wall -Werror -Wconversion -fno-builtin

# This hardcodes the path to libroll into the binary.
# This way we can run it directly without installing the library or setting LD_LIBRARY_PATH.
LD_RPATH := -Wl,-rpath,$(LIB_PATH)

#
# phony targets
#

.PHONY: all build check dynamic shared static clean distclean

all: check

build: $(PRODUCTS)

check: build dynamic shared static

dynamic: $(DYLIB_BIN) $(SHARED_LIB)
	./$(DYLIB_BIN) $(ARGS)

shared: $(SHARED_BIN) $(SHARED_LIB)
	./$(SHARED_BIN) $(ARGS)

static: $(STATIC_BIN)
	./$(STATIC_BIN) $(ARGS)

clean:
	rm -rf $(PRODUCTS)

distclean: clean
	cargo clean

#
# real targets
#

$(DYLIB_BIN): $(LIB_H) $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(DYLIB_BIN) -DDYNAMIC_ROLL

$(SHARED_BIN): $(LIB_H) $(SRC) $(SHARED_LIB)
	LIBRARY_PATH="$(LIB_PATH)" \
	  $(CC) $(CFLAGS) $(SRC) -o $(SHARED_BIN) $(LDFLAGS) $(LD_RPATH) -l $(LIB)

$(STATIC_BIN): $(LIB_H) $(SRC) $(STATIC_LIB)
	@# symlink $(LIB_FILE).a to $(LIB_FILE)-static.a
	@ln -sf "$(LIB_FILE).a" "$(LIB_PATH)/$(LIB_FILE)-static.a"
	LIBRARY_PATH="$(LIB_PATH)" \
	  $(CC) $(CFLAGS) $(SRC) -o $(STATIC_BIN) $(LDFLAGS) -l $(LIB)-static

$(SHARED_LIB) $(STATIC_LIB):
	cargo build --lib

