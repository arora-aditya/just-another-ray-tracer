COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
SOURCES := $(patsubst %.ppm,%.png,$(wildcard outputs/*.ppm))
.DEFAULT_GOAL := help

.PHONY: all bench build check clean doc install publish run test update help

all: run $(SOURCES)
	echo $(SOURCES)

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

tofile: build
	./target/debug/ray-tracing-in-a-weekend > outputs/the_next/01_moving_sphere_hq.ppm

test: build
	@$(CARGO) test

update:
	@$(CARGO) update

%.png: %.ppm
	magick convert $< $@

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'