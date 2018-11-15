
all: clippy test

format:
	@cargo fmt

clippy:
	@cargo clippy

test:
	@cargo test

build:
	@cargo build

run:
	@cargo run
	
doc:
	@cargo doc --open
