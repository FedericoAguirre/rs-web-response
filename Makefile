fmt :
	rustfmt src/lib.rs
	rustfmt tests/integration_test.rs

test : fmt
	cargo test

build : test
	cargo build -r

clean :
	rm -R target