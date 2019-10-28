PROGRAM=procedural_generation
HOMEWORK=project

CC=cargo build

FLAGS=--release

all: check_rust_version $(PROGRAM)

check_rust_version:
ifeq (, $(shell which cargo))
	$(error "No cargo in PATH, consider doing `apt-get install cargo` or better yet, use their recommended install method: `curl https://sh.rustup.rs -sSf | sh`")
endif

debug: check_rust_version $(PROGRAM)_debug

$(PROGRAM):
	$(CC) $(FLAGS) $(SRC_FILES)
	cp target/release/$(PROGRAM) $(HOMEWORK)

$(PROGRAM)_debug:
	$(CC) $(SRC_FILES)
	cp target/release/$(PROGRAM) $(HOMEWORK)_debug

.PHONY: clean
clean:
	cargo clean
	rm -rf $(PROGRAM) target/ $(HOMEWORK)


