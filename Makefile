
main: clean
	rustc main.rs
	RUST_BACKTRACE=full ./main

last:
	rustc main.rs
	RUST_BACKTRACE=full ./main last

clean:
	rm -f main
