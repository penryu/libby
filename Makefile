ARGS = 3 6

.PHONY: cdemo clean dyroll libroll rustroll
all: libroll rustroll dyroll cdemo

libroll:
	cargo build --lib

rustroll:
	cargo run --bin rustroll -- $(ARGS)

dyroll:
	cargo run --bin dyroll -- $(ARGS)

cdemo:
	gmake -C cdemo ARGS="$(ARGS)"

clean:
	cargo clean
	gmake -C cdemo clean
