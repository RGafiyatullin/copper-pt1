
.PHONY: all
all:

.PHONY: clean
clean:
	cargo clean

.PHONY: fmt
fmt:
	cargo +nightly fmt

.PHONY: doc
doc:
	cargo doc --all-features

.PHONY: clippy
clippy:
	cargo clippy --all-features

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: build-debug
build-debug:
	cargo build

.PHONY: test-release
test-release:
	cargo nextest run --release

.PHONY: test-debug
test-debug:
	cargo nextest run

