format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

build-container:
	//build the container
	docker build -t redact_ids .

release:
	cargo build --release

all: format lint test run
