SOURCES=$(wildcard src/*.rs)
PIPELINE_SOURCES=$(wildcard crates/*/src/*.rs)


all: $(SOURCES) Makefile
	cargo build
	cargo build --workspace

pipeline: $(PIPELINE_SOURCES) Makefile
	cargo build --workspace

rebuild:
	make clean
	make all

test:
	cargo test -- --show-output

clean:
	cargo clean
	cargo update

commit:
	aic -ac
	git push
