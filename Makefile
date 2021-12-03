
ifeq (run,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif

main:
	cargo run --release

run:
	cargo run --release $(RUN_ARGS)

last:
	cargo run last

clean:
	cargo clean

