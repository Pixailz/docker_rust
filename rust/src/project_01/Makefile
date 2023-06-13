ifeq ($(DEBUG),)
WERROR				:= "-D warnings"
else
WERROR				:=
endif

RUSTC				:= RUSTFLAGS=$(WERROR) cargo

run:				build
	$(RUSTC) run

build:
	$(RUSTC) build

check:
	$(RUSTC) check

clean:
	rm -rf ./target
	$(RUSTC) clean
