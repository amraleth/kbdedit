build:
	@echo "Building release:"
	@cargo build --release

setup:
	@echo "Moving executable to /usr/local/bin."
	@sudo mv target/release/kbdedit /usr/local/bin
