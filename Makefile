
main: clean
	rustc main.rs
	./main

last:
	rustc main.rs
	./main last

clean:
	rm -f main
