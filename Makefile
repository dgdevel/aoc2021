
main: clean
	cargo build --release
	cargo run --release

last:
	cargo build
	cargo run last

clean:
	cargo clean

