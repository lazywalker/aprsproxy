all: debug

debug:
	cargo build

release:
	cargo build --release

linux:
	cargo build --release --target x86_64-unknown-linux-musl

clean:
	cargo clean