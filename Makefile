SOURCES=$(wildcard src/*.rs)
TOOLS_SOURCES=$(wildcard crates/*/src/*.rs)


all: $(SOURCES) Makefile
	cargo build
	cargo build --workspace

tools: $(TOOLS_SOURCES) Makefile
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
