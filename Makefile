# rust-version:
# 	@echo "Rust command-line utility versions:"
# 	rustc --version 			#rust compiler
# 	cargo --version 			#rust package manager
# 	rustfmt --version			#rust code formatter
# 	rustup --version			#rust toolchain manager
# 	clippy-driver --version		#rust linter

# format:
# 	cargo fmt --quiet

# lint:
# 	cargo clippy --quiet

# test:
# 	cargo test --quiet

# run:
# 	cargo run

# release:
# 	cargo build --release

# all: format lint test run
install:
	cargo clean &&\
		cargo build -j 1

build:
	docker build -t movie .

rundocker:
	docker run -it --rm -p 8080:8080 movie

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run