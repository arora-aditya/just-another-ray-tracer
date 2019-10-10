COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)

.PHONY: all bench build check clean doc install publish run test update

all: run

bench:
	@$(CARGO) bench

build:
	@$(CARGO) build

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

install: build
	@$(CARGO) install

publish:
	@$(CARGO) publish

run: build
	./target/debug/ray-tracing-in-a-weekend

test: build
	@$(CARGO) test

update:
	@$(CARGO) update