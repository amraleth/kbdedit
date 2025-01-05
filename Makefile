build:
	@cargo build --release

setup:
	@sudo mv target/release/kbdedit /usr/local/bin
