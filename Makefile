GIT_VER=$(shell git rev-parse --short HEAD)
VER="0.2.1"
APP_VERSION=${VER}.${GIT_VER}
TARBALL="target/tarball"
APP_NAME="aprsproxy"

all: debug

debug:
	cargo build

macos:
	cargo build --release --target x86_64-apple-darwin

armv7:
	cargo build --release --target armv7-unknown-linux-musleabihf

linux:
	cargo build --release --target x86_64-unknown-linux-musl

clean:
	cargo clean

tarball: macos linux armv7
	mkdir -p ${TARBALL}
	tar cvfz "${TARBALL}/${APP_NAME}-${APP_VERSION}-x86_64-apple-darwin.tar.gz" -C target/x86_64-apple-darwin/release/ ${APP_NAME} 
	tar cvfz "${TARBALL}/${APP_NAME}-${APP_VERSION}-x86_64-unknown-linux-musl.tar.gz" -C target/x86_64-unknown-linux-musl/release/ ${APP_NAME}
	tar cvfz "${TARBALL}/${APP_NAME}-${APP_VERSION}-armv7-unknown-linux-musleabihf.tar.gz" -C target/armv7-unknown-linux-musleabihf/release/ ${APP_NAME}