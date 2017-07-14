# added to allow for linking with depricated openssl
build-arch:
	OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0 OPENSSL_LIB_DIR=/usr/lib/openssl-1.0 cargo build

build:
	cargo build


.PHONY: build build-arch
