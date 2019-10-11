COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
SOURCES := $(patsubst %.ppm,%.png,$(wildcard outputs/*.ppm))

.PHONY: all bench build check clean doc install publish run test update

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
	echo $(SOURCES)
	./target/debug/ray-tracing-in-a-weekend

tofile: build
	./target/debug/ray-tracing-in-a-weekend

test: build
	@$(CARGO) test

update:
	@$(CARGO) update

%.png: %.ppm
	magick convert $< $@