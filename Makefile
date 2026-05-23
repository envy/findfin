MUSL_TARGET = x86_64-unknown-linux-musl

.PHONY: build release static clean

build:
	cargo build

release:
	cargo build --release

static:
	cargo build --release --target $(MUSL_TARGET)

clean:
	cargo clean
