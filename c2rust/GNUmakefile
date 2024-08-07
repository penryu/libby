# A heavily commented Makefile.
#
# I've tried to make the logic fairly straightforward,
# and tried to add comments whenever it isn't.

PROJECT = c2rust
SRC := $(PROJECT).c

# default args; override with `ARGS="2 6"`
ARGS ?= 3 8

# get system type
SYSNAME := $(shell uname -s)

# system specific setup

ifeq ($(SYSNAME),FreeBSD)
	LDFLAGS += -l pthread
endif

ifeq ($(SYSNAME),Darwin)
	SHARED_LIB_EXT = dylib
else
	SHARED_LIB_EXT = so
endif

LIB := roll
LIB_SRC := src/lib.rs
LIB_FILE := lib$(LIB)
LIB_PATH := $(PWD)/target/debug
STATIC_LIB := $(LIB_PATH)/$(LIB_FILE).a
SHARED_LIB := $(LIB_PATH)/$(LIB_FILE).$(SHARED_LIB_EXT)

DYNAMIC_BIN := $(PROJECT)-dynamic
SHARED_BIN := $(PROJECT)-shared
STATIC_BIN := $(PROJECT)-static

BINS := $(STATIC_BIN) $(SHARED_BIN) $(DYNAMIC_BIN)
PRODUCTS := $(BINS) *.dSYM

# Try to keep our C safe.
CFLAGS += -g -O0 -Wall -Werror -Wconversion -fno-builtin

# This hardcodes the path to libroll into the binary.
# This way we can run it directly without installing the library or setting LD_LIBRARY_PATH.
LD_RPATH := -Wl,-rpath,$(LIB_PATH)

#
# phony targets
#

.PHONY: all build check clean distclean dynamic shared static

all: check

build: $(BINS)

check: build static shared dynamic

static: $(STATIC_BIN)
	./$(STATIC_BIN) $(ARGS)

shared: $(SHARED_BIN) $(SHARED_LIB)
	./$(SHARED_BIN) $(ARGS)

dynamic: $(DYNAMIC_BIN) $(SHARED_LIB)
	./$(DYNAMIC_BIN) $(ARGS)

clean:
	rm -rf $(PRODUCTS)

distclean: clean
	cargo clean

#
# real targets
#

# This is all handled by cargo. We consume the libraries right where cargo leaves them.
$(SHARED_LIB) $(STATIC_LIB): $(LIB_SRC)
	cargo build --lib

#
# static
#
# If `ld` finds both static and shared libraries, it will prefer shared.
# So to ensure this binary is statically linked, we symlink `libroll.a` to `libroll-static.a`,
# and pass `-l roll-static` to the linker to ensure we get a static binary.
#
# When we build the binary, the linker looks up the binary, finds the specific objects we want from
# `libroll-static.a`, and embeds them directly in our `c2rust-static` binary. We don't have to worry
# about whether it's installed when we launch.
#
$(STATIC_BIN): $(SRC) $(STATIC_LIB)
	ln -sf "$(LIB_FILE).a" "$(LIB_PATH)/$(LIB_FILE)-static.a"
	LIBRARY_PATH="$(LIB_PATH)" \
	  $(CC) $(CFLAGS) $(SRC) -o $(STATIC_BIN) $(LDFLAGS) -l $(LIB)-static

#
# shared
#
# No magic here. This is the default, smoothest, and to many, the preferred method of linking.
# `ld` finds the libraries at compile time and ensures they have the symbols we use. It then records
# our dependency on the library in the binary.
#
# When the binary is launched, the linker looks up the library on the path and attempts to load it
# before even launching the binary. If it's not found, the our demo doesn't even load.
#
$(SHARED_BIN): $(SRC) $(SHARED_LIB)
	LIBRARY_PATH="$(LIB_PATH)" \
	  $(CC) $(CFLAGS) $(SRC) -o $(SHARED_BIN) $(LDFLAGS) $(LD_RPATH) -l $(LIB)

#
# dynamic
#
# The most fragile method so far.
#
# There is no checking for the presence or validity of the library, neither at compile time nor at
# launch time. The system has no way of knowing you even want the library until the very moment we
# call `dlopen(3)`.
#
# Looking at the C code involved in dynamically loading the library, you can also see how much
# complexity it adds to your program logic.
#
# `DYLIB_PATH` is used to hardcode the path to the library inside the program. During the
# compilation, this value is also used to signal whether to include the `dlopen()` logic.
#
$(DYNAMIC_BIN): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(DYNAMIC_BIN) $(LDFLAGS) -DDYLIB_PATH="$(SHARED_LIB)"
