# Makefile styleguide at http://clarkgrubb.com/makefile-style-guide
MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := test
.DELETE_ON_ERROR:
.SUFFIXES:


UNAME?=$(shell uname)
PWD=$(shell pwd)

ifeq ($(UNAME), Linux)
  DYLIB_EXT=so
  TARGET_DIR=target
  STRIP=strip
endif
ifeq ($(UNAME), Darwin)
  DYLIB_EXT=dylib
  TARGET_DIR=target-$(UNAME)
  STRIP=strip -x
endif
CARGO:=CARGO_TARGET_DIR=$(TARGET_DIR) cargo
LIB:=$(TARGET_DIR)/release/libmidas.$(DYLIB_EXT)

.PHONY: test
test: out.csv test.out.csv
	diff -q $^

.PHONY: clean
clean:
	$(CARGO) clean
	rm -f midas.so test.out.csv

test.out.csv: test.py in.csv midas.so
	python3 test.py

midas.so: $(LIB)
	cp $< $@
	$(STRIP) $@

.PHONY: $(LIB)
$(LIB):
	$(CARGO) build --release
