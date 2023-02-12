run:
	./target/debug/metric URLs.txt

build:
	cargo build

api:
	python3 api.py

test:
	$ CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

install:
	cargo install grcov

