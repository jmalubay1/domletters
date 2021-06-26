prog :=domletters

build:
	cargo build --release

install:
	cp target/release/$(prog) ~/$(prog)

all: build install