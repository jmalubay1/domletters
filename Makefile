prog :=domletters

all: build copy

build:
	cargo build

install:
	cp target/debug/$(prog) .