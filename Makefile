all:
	cargo build --release

install:
	cargo install --path sv-add
	cargo install --path sv-del
	cargo install --path sv-list

clean:
	cargo clean
